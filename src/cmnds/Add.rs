pub mod add{
    use std::path::PathBuf;
    use crate::obj::db_entities::db_entities::{DBEntity, EntityType, store_db_entity};
    use crate::cmnds::Init::init::DBContainer;
    use crate::fs_abs::database::database::{Database, DatabaseType, create_database};

    #[derive(Debug, Clone)]
    enum AddTarget {
        Schema(Vec<String>),
        Data(Vec<String>),
        User(Vec<String>),
        Index(Vec<String>),
        Table(Vec<String>),
    }

    fn parse_add_args(args: &Vec<String>) -> (Option<String>, Vec<AddTarget>) {
        let mut container_name = None;
        let mut targets = Vec::new();

        for arg in args {
            if arg.starts_with("--container=") || arg.starts_with("--cont=") {
                container_name = Some(arg[arg.find("=").unwrap()+1..].to_string());
            } else if arg.contains("=") {
                let parts: Vec<&str> = arg.splitn(2, "=").collect();
                if parts.len() == 2 {
                    let entity_type = parts[0].to_uppercase();
                    let values = if parts[1] == "." {
                        vec![".".to_string()]
                    } else {
                        parts[1].split(",").map(|s| s.trim().to_string()).collect()
                    };

                    let target = match entity_type.as_str() {
                        "SCHEMA" => AddTarget::Schema(values),
                        "DATA" => AddTarget::Data(values),
                        "USER" => AddTarget::User(values),
                        "INDEX" => AddTarget::Index(values),
                        "TABLE" => AddTarget::Table(values),
                        _ => continue,
                    };
                    targets.push(target);
                }
            }
        }

        (container_name, targets)
    }

    fn load_config() -> crate::cmnds::Init::init::DB_conf {
        let config_path = PathBuf::from(".DBit/config.json");
        if !config_path.exists() {
            panic!("DBit not initialized");
        }
        let config_content = std::fs::read_to_string(config_path).unwrap();
        let config: crate::cmnds::Init::init::DB_conf = serde_json::from_str(&config_content).unwrap();
        config
    }

    fn get_container<'a>(containers: &'a Vec<DBContainer>, name: &str) -> Option<&'a DBContainer> {
        containers.iter().find(|c| c.name == name)
    }

    fn extract_schema(db: &mut Box<dyn Database>, table_name: &str, db_type: &DatabaseType) -> String {
        match db_type {
            DatabaseType::SQLite => {
                let query = format!("SELECT sql FROM sqlite_master WHERE type='table' AND name='{}'", table_name);
                if let Ok(results) = db.query(&query) {
                    if let Some(row) = results.first() {
                        if let Some(sql) = row.get("sql") {
                            return sql.clone();
                        }
                    }
                }
                format!("Unable to extract schema {}", table_name)
            }
            DatabaseType::MySQL => {
                let query = format!("SHOW CREATE TABLE {}", table_name);
                if let Ok(results) = db.query(&query) {
                    if let Some(row) = results.first() {
                        if let Some(create_table) = row.get("Create Table") {
                            return create_table.clone();
                        }
                    }
                }
                format!("Unable to extract schema {}", table_name)
            }
            DatabaseType::PostgreSQL => {
                let query = format!("SELECT column_name, data_type, is_nullable, column_default FROM information_schema.columns WHERE table_name = '{}' ORDER BY ordinal_position",table_name);
                if let Ok(results) = db.query(&query) {
                    let mut columns = Vec::new();
                    for row in results {
                        let col_name = row.get("column_name").unwrap_or(&"unknown".to_string()).clone();
                        let data_type = row.get("data_type").unwrap_or(&"text".to_string()).clone();
                        let nullable = row.get("is_nullable").unwrap_or(&"YES".to_string()).clone();
                        let default = row.get("column_default").unwrap_or(&"".to_string()).clone();
                        let col_def = if nullable == "NO" {
                            format!("{} {} NOT NULL DEFAULT {}", col_name, data_type, default)
                        } else {
                            format!("{} {} DEFAULT {}", col_name, data_type, default)
                        };
                        columns.push(col_def);
                    }
                    if !columns.is_empty() {
                        return format!("CREATE TABLE {} (\n  {}\n);", table_name, columns.join(",\n  "));
                    }
                }
                format!("Unable to extract schema {}", table_name)
            }
        }
    }

    fn extract_data(db: &mut Box<dyn Database>, table_name: &str) -> String {
        let query = format!("SELECT * FROM {}", table_name);
        if let Ok(results) = db.query(&query) {
            serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string())
        } else {
            "[]".to_string()
        }
    }

    fn extract_users(db: &mut Box<dyn Database>, db_type: &DatabaseType) -> String {
        match db_type {
            DatabaseType::SQLite => {
                if let Ok(results) = db.query("SELECT * FROM sqlite_master WHERE type='table'") {
                    serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string())
                } else {
                    "[]".to_string()
                }
            }
            DatabaseType::MySQL => {
                let query = "SELECT user, host FROM mysql.user";
                if let Ok(results) = db.query(query) {
                    serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string())
                } else {
                    "[]".to_string()
                }
            }
            DatabaseType::PostgreSQL => {
                let query = "SELECT usename, usecreatedb, usesuper FROM pg_user";
                if let Ok(results) = db.query(query) {
                    serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string())
                } else {
                    "[]".to_string()
                }
            }
        }
    }

    fn extract_indexes(db: &mut Box<dyn Database>, table_name: &str, db_type: &DatabaseType) -> String {
        match db_type {
            DatabaseType::SQLite => {
                let query = format!("SELECT * FROM sqlite_master WHERE type='index' AND tbl_name='{}'", table_name);
                if let Ok(results) = db.query(&query) {
                    serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string())
                } else {
                    "[]".to_string()
                }
            }
            DatabaseType::MySQL => {
                let query = format!("SHOW INDEX FROM {}", table_name);
                if let Ok(results) = db.query(&query) {
                    serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string())
                } else {
                    "[]".to_string()
                }
            }
            DatabaseType::PostgreSQL => {
                let query = format!(
                    "SELECT indexname, indexdef FROM pg_indexes WHERE tablename = '{}'",
                    table_name
                );
                if let Ok(results) = db.query(&query) {
                    serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string())
                } else {
                    "[]".to_string()
                }
            }
        }
    }

    fn get_all_tables(db: &mut Box<dyn Database>, db_type: &DatabaseType) -> Vec<String> {
        let mut tables = Vec::new();
        let query = match db_type {
            DatabaseType::SQLite => {"SELECT name FROM sqlite_master WHERE type='table'"}
            DatabaseType::MySQL => {"SHOW TABLES"}
            DatabaseType::PostgreSQL => {"SELECT tablename FROM pg_tables WHERE schemaname = 'public'"}
        };
        
        if let Ok(results) = db.query(query) {
            for row in results {
                let table_name = match db_type {
                    DatabaseType::SQLite => row.get("name").cloned(),
                    DatabaseType::MySQL => row.values().next().cloned(),
                    DatabaseType::PostgreSQL => row.get("tablename").cloned(),
                };
                if let Some(name) = table_name {
                    tables.push(name);
                }
            }
        }
        tables
    }

    pub fn add_cmnd(args: Vec<String>, _db_ignore: Option<Vec<String>>){
        let (container_name, targets) = parse_add_args(&args);
        if targets.is_empty() {
            println!("Usage: dbit add [SCHEMA|DATA|USER|INDEX|TABLE]=elem1,elem2,... ");
            println!("Example: dbit add SCHEMA=table1,table2 DATA=table1,table2 USER=user1,user2 INDEX=. TABLE=table1,table2");
            return;
        }
        let config = load_config();

        let container = if let Some(name) = container_name {
            get_container(&config.containers, &name).expect(&format!("Container '{}' not found", name))
        } else if config.containers.len() == 1 {
            &config.containers[0]
        } else {
            println!("Multiple containers found. Specify --container=<name>");
            for c in &config.containers {
                println!("  - {}", c.name);
            }
            return;
        };

        let mut db: Box<dyn Database> = create_database(container.db_type.clone());
        let username = container.user.as_deref().unwrap_or("");
        let password = container.pass.as_deref().unwrap_or("");
        let ip_str = match container.db_type {
            DatabaseType::SQLite => {container.path.as_deref().unwrap_or(".").to_string()}
            _ => container.host.as_deref().unwrap_or("localhost").to_string()
        };
        let db_name = match container.db_type {
            DatabaseType::SQLite => {container.path.as_deref().and_then(|p| std::path::Path::new(p).file_stem()).and_then(|s| s.to_str()).unwrap_or("db").to_string()}
            _ => container.database.as_deref().unwrap_or("").to_string()
        };
        
        if let Err(e) = db.connect(username, password, &ip_str, &db_name) {
            println!("Failed to connect to DB: {}", e);
            return;
        }

        let mut staged_entities: Vec<DBEntity> = Vec::new();
        let all_tables = get_all_tables(&mut db, &container.db_type);

        for target in targets {
            match target {
                AddTarget::Schema(tables) => {
                    let tables_to_add = if tables.contains(&".".to_string()) {
                        all_tables.clone()
                    } else {
                        tables
                    };

                    for table in tables_to_add {
                        let schema = extract_schema(&mut db, &table, &container.db_type);
                        let entity = DBEntity {container_name: container.name.clone(),entity_type: EntityType::Schema,name: table.clone(),content: schema,metadata: None};
                        let hash = store_db_entity(&entity);
                        staged_entities.push(entity);
                        println!("Added SCHEMA: {}", table);
                    }
                },
                AddTarget::Data(tables) => {
                    let tables_to_add = if tables.contains(&".".to_string()) {
                        all_tables.clone()
                    } else {
                        tables
                    };
                    for table in tables_to_add {
                        let data = extract_data(&mut db, &table);
                        let entity = DBEntity {container_name: container.name.clone(),entity_type: EntityType::Data,name: table.clone(),content: data,metadata: None};
                        let hash = store_db_entity(&entity);
                        staged_entities.push(entity);
                        println!("Added DATA for table: {}", table);
                    }
                }
                AddTarget::User(users) => {
                    let users_to_add = if users.contains(&".".to_string()) {
                        vec!["all".to_string()]
                    } else {
                        users
                    };

                    for user in users_to_add {
                        let user_data = extract_users(&mut db, &container.db_type);
                        let entity = DBEntity {container_name: container.name.clone(),entity_type: EntityType::User,name: user.clone(),content: user_data,metadata: None};
                        let hash = store_db_entity(&entity);
                        staged_entities.push(entity);
                        println!("Added User : {}", user);
                    }
                }
                AddTarget::Index(tables) => {
                    let tables_to_add = if tables.contains(&".".to_string()) {
                        all_tables.clone()
                    } else {
                        tables
                    };

                    for table in tables_to_add {
                        let indexes = extract_indexes(&mut db, &table, &container.db_type);
                        let entity = DBEntity {container_name: container.name.clone(),entity_type: EntityType::Index,name: table.clone(),content: indexes,metadata: None};
                        let hash = store_db_entity(&entity);
                        staged_entities.push(entity);
                        println!("Added INDEX for table: {}", table);
                    }
                }
                AddTarget::Table(tables) => {
                    let tables_to_add = if tables.contains(&".".to_string()) {
                        all_tables.clone()
                    } else {
                        tables
                    };
                    for table in tables_to_add {
                        let schema = extract_schema(&mut db, &table, &container.db_type);
                        let data = extract_data(&mut db, &table);
                        let schema_entity = DBEntity {container_name: container.name.clone(),entity_type: EntityType::Schema,name: table.clone(),content: schema,metadata: None};
                        let schema_hash = store_db_entity(&schema_entity);
                        staged_entities.push(schema_entity);

                        let data_entity = DBEntity {container_name: container.name.clone(),entity_type: EntityType::Data,name: table.clone(),content: data,metadata: None};

                        let data_hash = store_db_entity(&data_entity);
                        staged_entities.push(data_entity);

                        println!("Added Schama and Data for Table: {}", table);
                    }
                }
            }
        }

        let staging_path = PathBuf::from(".DBit").join("staging_db");
        let serialized = serde_json::to_string(&staged_entities).unwrap();
        std::fs::write(staging_path, serialized).unwrap();

        println!("Staged {} entities", staged_entities.len());
    }

}