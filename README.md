# DBit

A Git-like version control system for databases. DBit allows you to track, version, and manage database schemas, data, users, and indexes across multiple database types including SQLite, PostgreSQL, and MySQL.

## Why DBit?

Traditional version control systems like Git are designed for source code files, but databases present unique challenges:

- **Schema Evolution**: Track changes to table structures, columns, and constraints over time
- **Data Versioning**: Capture and restore database states at specific points in time
- **Multi-Database Support**: Manage heterogeneous database environments (SQLite, PostgreSQL, MySQL) in a single repository
- **Branching & Merging**: Experiment with database changes in isolation before deploying
- **Audit Trail**: Maintain a complete history of who changed what and when

DBit solves these problems by providing Git-like semantics for database entities, enabling teams to collaborate on database changes with confidence.

## Features

- **Multi-Database Support**: SQLite, PostgreSQL, and MySQL
- **Entity Tracking**: Version schemas, data, users, indexes, and entire tables
- **Git-like Workflow**: init, add, commit, branch, checkout, push, pull, status, diff
- **Branch Management**: Create and switch between database configuration branches
- **Staging Area**: Stage database entities before committing
- **Snapshot Storage**: Efficient storage of database states using content-addressable storage
- **Configuration Management**: Declarative database configuration via `.dbio` files

## Dependencies

DBit is written in Rust and requires the following dependencies:

```toml
rand = "0.8"
sha2 = "0.10"
hex = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
rusqlite = { version = "0.31", features = ["bundled"] }
mysql = "25"
postgres = "0.19"
```

### Build Requirements

- Rust 2024 edition or later
- Cargo package manager

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd DBit

# Build the project
cargo build --release

# The binary will be available at target/release/DBit
```

### Development Build

```bash
cargo build
```

The debug binary will be available at `target/debug/DBit`.

## Configuration

Before initializing a DBit repository, you need to create a configuration file (`config.dbio`) that defines your database containers.

### Example config.dbio

```dbio
# SQLite database
Sqlite(path="./data/local.db",name="local_cache")
@Init
    init: [users(profiles,sessions), logs(events,errors), config]
@Monitor
    database: [users, logs]

# PostgreSQL database
PostgreSQL(host="10.0.0.5:5432",user="admin",pass="s3cr3t",database="main",name="primary_pg")
@Init
    init: [orders(line_items), customers]
@Monitor
    database: [orders, customers]

# MySQL database
Mysql(host="db.internal:3306",user="root",pass="rootpass",database="analytics",name="analytics_db")
@Observe
    database: [events, aggregates, reports]
```

### Configuration Syntax

- **Container Declaration**: `DatabaseType(param="value",name="container_name")`
- **@Init Section**: Defines databases and tables to initialize
- **@Monitor/@Observe Section**: Defines databases to track for changes

Supported database types:
- `Sqlite(path="...",name="...")`
- `PostgreSQL(host="...",user="...",pass="...",database="...",name="...")`
- `Mysql(host="...",user="...",pass="...",database="...",name="...")`

## Usage

### Initialize a Repository

```bash
# Initialize with default config
dbit init --config=config.dbio --author="Your Name"

# Initialize with custom author
dbit init --config=config.dbio --author="John Doe"
```

This creates a `.DBit` directory with the repository structure and stores the configuration.

### Add Database Entities

Stage database entities for commit:

```bash
# Add schema for specific tables
dbit add SCHEMA=table1,table2

# Add data for specific tables
dbit add DATA=table1,table2

# Add all tables (using .)
dbit add SCHEMA=. DATA=.

# Add users
dbit add USER=all

# Add indexes
dbit add INDEX=table1,table2

# Add entire tables (schema + data)
dbit add TABLE=table1,table2

# Specify container when multiple exist
dbit add --container=primary_pg SCHEMA=orders
```

### Commit Changes

```bash
# Commit with a message
dbit commit "Added orders table schema"

# Commit with default message
dbit commit
```

### Branch Management

```bash
# List all branches
dbit branch

# Create a new branch
dbit branch feature-branch

# Create and switch to a new branch
dbit branch -M feature-branch

# Switch to an existing branch
dbit branch -S main
```

### Checkout

Switch branches and restore database state:

```bash
# Checkout a branch
dbit checkout feature-branch
```

This restores files and database entities to the state of the specified branch.

### Status

Check the current state:

```bash
dbit status
```

Shows:
- Current branch
- Last commit
- Staged files
- Staged database entities

### Push/Pull

Move changes between branches:

```bash
# Push changes from one branch to another
dbit push feature-branch main

# Pull changes from one branch to another
dbit pull main feature-branch
```

### Diff

View differences:

```bash
# View database container differences
dbit diff --db primary_pg
```

### Debug Mode

Enable debug output:

```bash
dbit -d init --config=config.dbio
dbit add -d SCHEMA=.
```

### Help

```bash
dbit -h
dbit --help
```

## Examples

### Complete Workflow

```bash
# 1. Initialize repository
dbit init --config=config.dbio --author="Alice"

# 2. Add initial schema
dbit add SCHEMA=users,products

# 3. Commit
dbit commit "Initial schema"

# 4. Create a feature branch
dbit branch -M add-orders

# 5. Add new table schema
dbit add SCHEMA=orders

# 6. Commit the change
dbit commit "Added orders table"

# 7. Switch back to main
dbit branch -S main

# 8. Pull the feature changes
dbit pull add-orders main

# 9. Check status
dbit status
```

### Multi-Container Workflow

```bash
# Add schema from specific container
dbit add --container=local_cache SCHEMA=users
dbit add --container=primary_pg SCHEMA=orders

# Commit both
dbit commit "Added schemas from multiple containers"

# View status
dbit status
```

### Database Migration Workflow

```bash
# 1. Branch for migration
dbit branch -M migration-v2

# 2. Add current schema
dbit add SCHEMA=.

# 3. Commit baseline
dbit commit "Baseline schema before migration"

# 4. Apply migration to database
# (Apply your SQL migration manually)

# 5. Add new schema
dbit add SCHEMA=.

# 6. Commit migration
dbit commit "Applied migration v2"

# 7. Switch back to main
dbit branch -S main
```

## Repository Structure

After initialization, DBit creates the following structure:

```
.DBit/
├── HEAD                    # Current branch reference
├── config.json             # Repository configuration
├── objects/                # Content-addressable object storage
│   ├── blobs/             # File contents
│   ├── trees/             # Directory structures
│   ├── commits/           # Commit objects
│   └── db_entities/       # Database entity objects
├── refs/                   # References
│   └── heads/             # Branch references
├── staging                 # Staged files
└── staging_db             # Staged database entities
```

## Advanced Usage

### .DBignore File

Create a `.DBignore` file to exclude certain database entities from tracking:

```
# Example .DBignore
temp_*
test_*
```

### Multiple Database Types

DBit can manage heterogeneous database environments in a single repository, allowing you to:

- Track SQLite for local development
- Track PostgreSQL for staging
- Track MySQL for production

All within the same version control workflow.

## License

This project is licensed under GPL-3.0-only. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please ensure your code compiles with `cargo check` and follows the existing code style.

## Troubleshooting

### "DBit not initialized"
Run `dbit init` first to initialize the repository.

### "Container not found"
Ensure the container name matches your `config.dbio` file.

### "Failed to connect to DB"
Check your database credentials in the configuration and ensure the database is accessible.

### "Nothing to commit"
Use `dbit add` to stage database entities before committing.
