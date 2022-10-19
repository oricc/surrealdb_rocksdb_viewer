# SurrealDB RocksDB Viewer

This small program allows for viewing the rocksDB backing SurrealDB.

## How to run

1. Start a SurrealDB instance backed by a RocksDB Database:

```cmd
surreal start rocksdb://local.db --user root --pass root
```

2. Open a CLI
```cmd
surreal sql --conn http://0.0.0.0:8000 --user root --pass root --ns test --db test
```

3. Insert some data 

```sql
-- create new tables and records
CREATE user:oricc SET name="Ori";
CREATE user:tobie SET name="Tobie";
CREATE repo:surrealdb SET name="SurrealDB";

-- Add some relations
RELATE user:tobie->contributes->repo:surrealdb;
RELATE user:ori->contributes->repo:surrealdb;
```

3. Create the environment variable

```cmd
SURREALDB_PATH=./local.db
```

4. Run the program

```cmd
cargo run
```
