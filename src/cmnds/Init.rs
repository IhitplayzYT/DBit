pub mod init{
    use std::{collections::HashMap, fs};
    use serde::{Serialize, Deserialize};
    use crate::fs_abs::{Fs_basic::fs_basic::cwd_contains, database::database::DatabaseType};

    #[derive(Debug,Clone,Serialize,Deserialize,PartialEq, Eq,Hash)]
    pub struct DBContainer {
        pub name: String,
        pub db_type: DatabaseType,
        pub path: Option<String>,
        pub host: Option<String>,
        pub user: Option<String>,
        pub pass: Option<String>,
        pub database: Option<String>,
    }

    #[derive(Debug,Clone,Serialize,Deserialize)]
    pub struct DB_conf{
        pub containers: Vec<DBContainer>,
        pub author: String,
        pub inits: HashMap<DBContainer, Vec<Inits>>,
        pub monitors: HashMap<DBContainer, Vec<String>>,
    }

    #[derive(Debug,Clone,Serialize,Deserialize)]
    pub enum Inits{
        Database(String),
        Table(String,String)
    }

    impl DB_conf{
        pub fn new(author: Option<String>) -> Self{
            Self {
                containers: vec![],
                author: author.unwrap_or("unknown".to_string()),
                inits: HashMap::new(),
                monitors: HashMap::new(),
            }
        }
    }

    pub enum Header{
        Init,
        Monitor
    }

    /// (`path="",name="x"`)
    /// (`d1(t1),d2(t2,t3),d3`).
    fn split_top_level(s: &str, delim: char) -> Vec<String> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut depth: i32 = 0;
        for c in s.chars() {
            match c {
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(c);
                }
                '(' | '[' if !in_quotes => {
                    depth += 1;
                    current.push(c);
                }
                ')' | ']' if !in_quotes => {
                    depth -= 1;
                    current.push(c);
                }
                c if c == delim && !in_quotes && depth == 0 => {
                    parts.push(current.clone());
                    current.clear();
                }
                _ => current.push(c),
            }
        }
        if !current.trim().is_empty() {
            parts.push(current);
        }
        parts
    }

    fn strip_quotes(s: &str) -> String {
        let s = s.trim();
        if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') {
            s[1..s.len() - 1].to_string()
        } else {
            s.to_string()
        }
    }

    /// Parses a container declaration line like:
    /// `PostgreSQL(host="",user="",pass="",database="",name="ident2")`
    fn parse_container_line(line: &str) -> DBContainer {
        let line = line.trim();
        let open = line.find('(').unwrap_or_else(|| {
            panic!("Malformed config: expected '(' in container declaration '{}'", line)
        });
        let close = line.rfind(')').unwrap_or_else(|| {
            panic!("Malformed config: expected ')' in container declaration '{}'", line)
        });

        let type_name = line[..open].trim().to_lowercase();
        let db_type = match type_name.as_str() {
            "sqlite" => DatabaseType::SQLite,
            "postgresql" | "postgres" => DatabaseType::PostgreSQL,
            "mysql" => DatabaseType::MySQL,
            _ => panic!("Malformed config: unsupported database type '{}'", type_name),
        };

        let args_str = &line[open + 1..close];
        let mut fields: HashMap<String, String> = HashMap::new();
        for part in split_top_level(args_str, ',') {
            let part = part.trim();
            if part.is_empty() { continue; }
            let eq = part.find('=').unwrap_or_else(|| {
                panic!("Malformed config: expected '=' in field '{}'", part)
            });
            let key = part[..eq].trim().to_lowercase();
            let value = strip_quotes(&part[eq + 1..]);
            fields.insert(key, value);
        }

        let name = fields.remove("name").unwrap_or_else(|| {
            panic!("Malformed config: container missing 'name' field in '{}'", line)
        });

        let host = fields.remove("host").or_else(|| fields.remove("ip_port"));

        DBContainer {name,db_type,path: fields.remove("path").filter(|s| !s.is_empty()),host: host.filter(|s| !s.is_empty()),user: fields.remove("user").filter(|s| !s.is_empty()),pass: fields.remove("pass").filter(|s| !s.is_empty()),database: fields.remove("database").filter(|s| !s.is_empty())}
    }

    /// Splits key: value into (key, value)
    fn split_key_value(line: &str) -> (String, String) {
        let idx = line.find(':').unwrap_or_else(|| {
            panic!("Malformed config: expected ':' in '{}'", line)
        });
        (line[..idx].trim().to_lowercase(), line[idx + 1..].trim().to_string())
    }

    fn parse_bracket_list(value: &str) -> Vec<String> {
        let value = value.trim();
        if !value.starts_with('[') || !value.ends_with(']') {
            panic!("Malformed config: expected a '[...]' list, got '{}'", value);
        }
        let inner = &value[1..value.len() - 1];
        split_top_level(inner, ',').into_iter().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
    }

    fn parse_init_line(line: &str, container: &DBContainer, conf: &mut DB_conf) {
        let (key, value) = split_key_value(line);
        match key.as_str() {
            "init" => {
                let entry = conf.inits.entry(container.clone()).or_insert_with(Vec::new);
                for item in parse_bracket_list(&value) {
                    if let Some(open) = item.find('(') {
                        let close = item.rfind(')').unwrap_or_else(|| {
                            panic!("Malformed config: expected ')' in init entry '{}'", item)
                        });
                        let db_name = item[..open].trim().to_string();
                        let tables_str = &item[open + 1..close];
                        entry.push(Inits::Database(db_name.clone()));
                        for table in split_top_level(tables_str, ',') {
                            let table = table.trim();
                            if !table.is_empty() {
                                entry.push(Inits::Table(db_name.clone(), table.to_string()));
                            }
                        }
                    } else {
                        entry.push(Inits::Database(item));
                    }
                }
            }
            other => panic!("Malformed config: unsupported key '{}' under @Init", other),
        }
    }

    fn parse_monitor_line(line: &str, container: &DBContainer, conf: &mut DB_conf) {
        let (key, value) = split_key_value(line);
        match key.as_str() {
            "database" => {
                let entry = conf.monitors.entry(container.clone()).or_insert_with(Vec::new);
                for item in parse_bracket_list(&value) {
                    entry.push(item);
                }
            }
            other => panic!("Malformed config: unsupported key '{}' under @Monitor", other),
        }
    }

    fn parse_config_file(config_path: &str) -> DB_conf {
        let config_content = fs::read_to_string(config_path).unwrap_or_else(|_| {
            panic!("Config file not found: {}", config_path)
        });
        let mut conf = DB_conf::new(Some("unknown".to_string()));
        let mut header: Option<Header> = None;
        let mut curr_container: Option<DBContainer> = None;

        for raw_line in config_content.lines() {
            let line = raw_line.trim_end();
            if line.trim().is_empty() {
                curr_container = None;
                header = None;
                continue;
            }

            // Indented line -> belongs to the active header + container.
            if raw_line.starts_with(' ') || raw_line.starts_with('\t') || raw_line.starts_with("  ") {
                let trimmed = line.trim();
                let container = curr_container.clone().unwrap_or_else(|| {
                    panic!("Malformed config: indented line with no active container: '{}'", raw_line)
                });
                match &header {
                    Some(Header::Init) => parse_init_line(trimmed, &container, &mut conf),
                    Some(Header::Monitor) => parse_monitor_line(trimmed, &container, &mut conf),
                    None => panic!("Malformed config: indented line with no active header: '{}'", raw_line),
                }
                continue;
            }

            if line.starts_with('@') {
                if curr_container.is_none() {
                    panic!("Malformed config: header '{}' found with no preceding container", line);
                }
                let hr = line.trim_start_matches('@').trim().to_lowercase();
                header = match hr.as_str() {
                    "init" => Some(Header::Init),
                    "monitor" | "observe" => Some(Header::Monitor),
                    _ => panic!("Malformed config: unsupported header '{}'", line),
                };
                continue;
            }

            let container = parse_container_line(line);
            conf.containers.push(container.clone());
            curr_container = Some(container);
            header = None;
        }

        conf
    }

    fn create_dbit_dir(){
        fs::create_dir(".DBit").unwrap();
        std::env::set_current_dir(".DBit").unwrap();
        fs::write("HEAD","refs/heads/main").unwrap();
        fs::create_dir_all("refs/heads").unwrap();
        fs::write("refs/heads/main","").unwrap();
        fs::create_dir("objects").unwrap();
        std::env::set_current_dir("..").unwrap();
    }

    pub fn init_cmnd(args: Vec<String>){
        if cwd_contains(".DBit"){
            panic!("The current Dir is already a DBit Repo");
        }
        let mut config_path = "config.dbio".to_string();
        let mut author = "unknown".to_string();
        for arg in args {
            if arg.starts_with("--config=") || arg.starts_with("--conf=") {
                config_path = arg[arg.find("=").unwrap()+1..].to_string();
            } else if arg.starts_with("--author=") || arg.starts_with("--name=") {
                author = arg[arg.find("=").unwrap()+1..].to_string();
            }
        }
        let mut conf = parse_config_file(&config_path);
        conf.author = author;
        create_dbit_dir();
        let config_serialized = serde_json::to_string_pretty(&conf).unwrap();
        fs::write(".DBit/config.json", config_serialized).unwrap();
        println!("Initialized DBit repo with {} containers", conf.containers.len());
        for container in &conf.containers {
            println!("  - {} ({:?})", container.name, container.db_type);
        }
    }

}
