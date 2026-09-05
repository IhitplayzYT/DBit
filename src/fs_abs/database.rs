pub mod database {
    use std::collections::HashMap;
    use mysql::prelude::Queryable;
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum DatabaseType {
        SQLite,
        MySQL,
        PostgreSQL,
    }

    pub trait Database {
        fn connect(&mut self, username: &str, pass: &str, ip_str: &str, db: &str) -> Result<(), String>;
        fn disconnect(&mut self) -> Result<(), String>;
        fn execute(&mut self, query: &str) -> Result<(), String>;
        fn query(&mut self, query: &str) -> Result<Vec<HashMap<String, String>>, String>;
        fn insert(&mut self, table: &str, data: HashMap<String, String>) -> Result<(), String>;
        fn select(&mut self, table: &str, conditions: Option<HashMap<String, String>>) -> Result<Vec<HashMap<String, String>>, String>;
        fn update(&mut self, table: &str, data: HashMap<String, String>, conditions: HashMap<String, String>) -> Result<(), String>;
        fn delete(&mut self, table: &str, conditions: HashMap<String, String>) -> Result<(), String>;
    }

    pub struct SQLiteDatabase {
        connection: Option<rusqlite::Connection>,
    }

    impl SQLiteDatabase {
        pub fn new() -> Self {
            Self { connection: None }
        }
    }

    impl Database for SQLiteDatabase {
        fn connect(&mut self,_username: &str,_pass: &str,ip_str: &str,db: &str) -> Result<(), String> {
            match rusqlite::Connection::open(format!("{ip_str}/{db}.db")) {
                Ok(conn) => {
                    self.connection = Some(conn);
                    Ok(())
                }
                Err(e) => Err(format!("SQLlite connection error: {}", e))
            }
        }

        fn disconnect(&mut self) -> Result<(), String> {
            self.connection = None;
            Ok(())
        }

        fn execute(&mut self, query: &str) -> Result<(), String> {
            if let Some(conn) = &self.connection {
                conn.execute(query, []).map_err(|e| format!("SQLlite execution error: {}", e))?;
                Ok(())
            } else {
                Err("Not connected to DB".to_string())
            }
        }

        fn query(&mut self, query: &str) -> Result<Vec<HashMap<String, String>>, String> {
            if let Some(conn) = &self.connection {
                let mut stmt = conn.prepare(query).map_err(|e| format!("SQLlite prepare error: {}", e))?;
                let mut rows = stmt.query([]).map_err(|e| format!("SQLlite query error: {}", e))?;
                let mut results = Vec::new();
                while let Some(row) = rows.next().map_err(|e| format!("SQLlite row error: {}", e))? {
                    let mut map = HashMap::new();
                    for i in 0..row.as_ref().column_count() {
                        map.insert(row.as_ref().column_name(i).unwrap_or("").to_string(), row.get(i).unwrap_or_default());
                    }
                    results.push(map);
                }
                Ok(results)
            } else {
                Err("Not connected to DB".to_string())
            }
        }

        fn insert(&mut self, table: &str, data: HashMap<String, String>) -> Result<(), String> {
            let columns: Vec<&str> = data.keys().map(|k| k.as_str()).collect();
            let placeholders: Vec<&str> = vec!["?"; columns.len()];
            let values: Vec<&String> = data.values().collect();
            let query = format!("INSERT INTO {} ({}) VALUES ({})",table,columns.join(", "),placeholders.join(", "));
            
            if let Some(conn) = &self.connection {
                let mut stmt = conn.prepare(&query).map_err(|e| format!("SQLlite execution error: {}", e))?;
                let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
                for v in &values {
                    params.push(v);
                }
                stmt.execute(params.as_slice()).map_err(|e| format!("SQLlite execution error: {}", e))?;
                Ok(())
            } else {
                Err("Not connected to DB".to_string())
            }
        }

        fn select(&mut self, table: &str, conditions: Option<HashMap<String, String>>) -> Result<Vec<HashMap<String, String>>, String> {
            let query = if let Some(conds) = conditions {
                let cond: Vec<String> = conds.iter().map(|(k, v)| format!("{} = '{}'", k, v)).collect();
                format!("SELECT * FROM {} WHERE {}", table, cond.join(" AND "))
            } else {
                format!("SELECT * FROM {}", table)
            };
            
            self.query(&query)
        }

        fn update(&mut self, table: &str, data: HashMap<String, String>, conditions: HashMap<String, String>) -> Result<(), String> {
            let set_clause: Vec<String> = data.iter().map(|(k, v)| format!("{} = '{}'", k, v)).collect();
            let cond: Vec<String> = conditions.iter().map(|(k, v)| format!("{} = '{}'", k, v)).collect();            
            let query = format!("UPDATE {} SET {} WHERE {}",table,set_clause.join(", "),cond.join(" AND "));   
            self.execute(&query)
        }

        fn delete(&mut self, table: &str, conditions: HashMap<String, String>) -> Result<(), String> {
            let cond: Vec<String> = conditions.iter().map(|(k, v)| format!("{} = '{}'", k, v)).collect();
            let query = format!("DELETE FROM {} WHERE {}", table, cond.join(" AND "));
            self.execute(&query)
        }
    }

    pub struct MySQLDatabase {
        connection: Option<mysql::Conn>,
    }

    impl MySQLDatabase {
        pub fn new() -> Self {
            Self { connection: None }
        }
    }

    impl Database for MySQLDatabase {
        fn connect(&mut self,username: &str,pass:&str,ip_port:&str,db: &str) -> Result<(), String> {
            let url = &format!("mysql://{username}:{pass}@{ip_port}/{db}")[..];
            match mysql::Conn::new(url) {
                Ok(conn) => {
                    self.connection = Some(conn);
                    Ok(())
                }
                Err(e) => Err(format!("MySQL connection error: {}", e))
            }
        }

        fn disconnect(&mut self) -> Result<(), String> {
            self.connection = None;
            Ok(())
        }

        fn execute(&mut self, query: &str) -> Result<(), String> {
            if let Some(conn) = &mut self.connection {
                conn.query_drop(query).map_err(|e| format!("MySQL execution error: {}", e))?;
                Ok(())
            } else {
                Err("Not connected to DB".to_string())
            }
        }

        fn query(&mut self, query: &str) -> Result<Vec<HashMap<String, String>>, String> {
            if let Some(conn) = &mut self.connection {
                let mut results = Vec::new();                
                let rows = conn.query_iter(query).map_err(|e| format!("MySQL query error: {}", e))?;
                for row_result in rows {
                    let row = row_result.map_err(|e| format!("MySQL row error: {}", e))?;
                    let mut map = HashMap::new();
                    for (i, column) in row.columns_ref().iter().enumerate() {
                        map.insert(column.name_str().to_string(), row.get(i).unwrap_or_default());
                    }
                    results.push(map);
                }
                Ok(results)
            } else {
                Err("Not connected to DB".to_string())
            }
        }

        fn insert(&mut self, table: &str, data: HashMap<String, String>) -> Result<(), String> {
            let columns: Vec<&str> = data.keys().map(|k| k.as_str()).collect();
            let values: Vec<String> = data.values().cloned().collect();            
            let query = format!("INSERT INTO {} ({}) VALUES ('{}')",table,columns.join(", "),values.join("', '"));
            self.execute(&query)
        }

        fn select(&mut self, table: &str, conditions: Option<HashMap<String, String>>) -> Result<Vec<HashMap<String, String>>, String> {
            let query = if let Some(conds) = conditions {
                let cond: Vec<String> = conds.iter().map(|(k, v)| format!("{} = '{}'", k, v)).collect();
                format!("SELECT * FROM {} WHERE {}", table, cond.join(" AND "))
            } else {
                format!("SELECT * FROM {}", table)
            };
            
            self.query(&query)
        }

        fn update(&mut self, table: &str, data: HashMap<String, String>, conditions: HashMap<String, String>) -> Result<(), String> {
            let set_clause: Vec<String> = data.iter().map(|(k, v)| format!("{} = '{}'", k, v)).collect();
            let cond: Vec<String> = conditions.iter().map(|(k, v)| format!("{} = '{}'", k, v)).collect();
            let query = format!("UPDATE {} SET {} WHERE {}",table,set_clause.join(", "),cond.join(" AND "));
            self.execute(&query)
        }

        fn delete(&mut self, table: &str, conditions: HashMap<String, String>) -> Result<(), String> {
            let cond: Vec<String> = conditions.iter().map(|(k, v)| format!("{} = '{}'", k, v)).collect();
            let query = format!("DELETE FROM {} WHERE {}", table, cond.join(" AND "));
            self.execute(&query)
        }
    }

    pub struct PostgreSQLDatabase {
        connection: Option<postgres::Client>,
    }

    impl PostgreSQLDatabase {
        pub fn new() -> Self {
            Self { connection: None }
        }
    }

    impl Database for PostgreSQLDatabase {
            fn connect(&mut self,username: &str,pass: &str,ip_str: &str,db: &str) -> Result<(), String> {
            let conn_str = &format!("host={ip_str} user={username} password={pass} dbname={db}")[..];
            match postgres::Client::connect(conn_str, postgres::NoTls) {
                Ok(client) => {
                    self.connection = Some(client);
                    Ok(())
                }
                Err(e) => Err(format!("PostgreSQL connection error: {}", e))
            }
        }

        fn disconnect(&mut self) -> Result<(), String> {
            self.connection = None;
            Ok(())
        }

        fn execute(&mut self, query: &str) -> Result<(), String> {
            if let Some(client) = &mut self.connection {
                client.execute(query, &[]).map_err(|e| format!("PostgreSQL execution error: {}", e))?;
                Ok(())
            } else {
                Err("Not connected to DB".to_string())
            }
        }

        fn query(&mut self, query: &str) -> Result<Vec<HashMap<String, String>>, String> {
            if let Some(client) = &mut self.connection {
                let mut results = Vec::new();
                for row in client.query(query, &[]).map_err(|e| format!("PostgreSQL query error: {}", e))? {
                    let mut map = HashMap::new();
                    for i in 0..row.len() {
                        if let Some(name) = row.columns().get(i) {
                            map.insert(name.name().to_string(), row.get(i));
                        }
                    }
                    results.push(map);
                }
                Ok(results)
            } else {
                Err("Not connected to DB".to_string())
            }
        }

        fn insert(&mut self, table: &str, data: HashMap<String, String>) -> Result<(), String> {
            let columns: Vec<&str> = data.keys().map(|k| k.as_str()).collect();
            let values: Vec<String> = data.values().cloned().collect();            
            let query = format!("INSERT INTO {} ({}) VALUES ($1)",table,columns.join(", "));
            if let Some(client) = &mut self.connection {
                client.execute(&query, &[&values[0]]).map_err(|e| format!("PostgreSQL execute error: {}", e))?;
                Ok(())
            } else {
                Err("Not connected to DB".to_string())
            }
        }

        fn select(&mut self, table: &str, conditions: Option<HashMap<String, String>>) -> Result<Vec<HashMap<String, String>>, String> {
            let query = if let Some(conds) = conditions {
                let cond: Vec<String> = conds.iter().map(|(k, v)| format!("{} = '{}'", k, v)).collect();
                format!("SELECT * FROM {} WHERE {}", table, cond.join(" AND "))
            } else {
                format!("SELECT * FROM {}", table)
            };
            self.query(&query)
        }

        fn update(&mut self, table: &str, data: HashMap<String, String>, conditions: HashMap<String, String>) -> Result<(), String> {
            let set_clause: Vec<String> = data.iter().map(|(k, v)| format!("{} = '{}'", k, v)).collect();
            let cond: Vec<String> = conditions.iter().map(|(k, v)| format!("{} = '{}'", k, v)).collect();
            let query = format!("UPDATE {} SET {} WHERE {}",table,set_clause.join(", "),cond.join(" AND "));
            self.execute(&query)
        }

        fn delete(&mut self, table: &str, conditions: HashMap<String, String>) -> Result<(), String> {
            let cond: Vec<String> = conditions.iter().map(|(k, v)| format!("{} = '{}'", k, v)).collect();
            let query = format!("DELETE FROM {} WHERE {}", table, cond.join(" AND "));
            self.execute(&query)
        }
    }

    pub fn create_database(db_type: DatabaseType) -> Box<dyn Database> {
        match db_type {
            DatabaseType::SQLite => Box::new(SQLiteDatabase::new()),
            DatabaseType::MySQL => Box::new(MySQLDatabase::new()),
            DatabaseType::PostgreSQL => Box::new(PostgreSQLDatabase::new()),
        }
    }
}
