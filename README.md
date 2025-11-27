# sqlite-fsr

A from-scratch SQLite database file reader and SQL parser written in Rust as a learning project to understand database internals.

## Learning Objectives

This project is designed as an **educational exploration** of two fundamental database concepts:

1. **Binary Database File Format** — Understanding how SQLite stores data on disk
2. **SQL Query Parsing** — Building a tokenizer and Abstract Syntax Tree (AST) for SQL statements

The implementation is intentionally minimal and focused on clarity over performance, making it ideal for learning how databases work under the hood.

## Architecture Overview

### 1. SQLite Data Storage Format → Codebase Mapping

SQLite uses a well-documented binary file format. This project implements the core structures:

#### **Database Pages** (`src/models/dbfile/`)
- SQLite organizes data into fixed-size pages (typically 4096 bytes)
- **Implementation**: `schema/schemaraw.rs` - Parses the database header and schema page
- **Key concept**: B-tree structure with interior and leaf pages

#### **B-Tree Pages** (`src/models/dbfile/dbtable/tablepage/`)
- `LeafTablePage` - Contains actual table records (the data rows)
- `InteriorTablePage` - Contains pointers to child pages for navigation
- **Learn**: How databases achieve O(log n) lookups through tree structures

#### **Records & Serialization** (`src/models/dbfile/dbtable/tablepage/record.rs`)
- Records use SQLite's serial type system to store typed data as byte arrays
- **Implementation**: `TableRow` converts raw bytes (`Vec<u8>`) into typed `String` values
- **Key concept**: How databases serialize/deserialize typed data to binary format

#### **Variable-Length Integers (Varints)** (`src/utils/varint.rs`)
- SQLite uses space-efficient varint encoding (1-9 bytes) for integers
- **Learn**: How databases optimize storage by using variable-length encodings

### 2. SQL Parsing → Codebase Mapping

SQL queries are transformed from text into executable operations through parsing:

#### **Tokenization** (`src/command/sql/parser/sql_token.rs`)
```rust
"SELECT name FROM users" → [Keyword("SELECT"), Identifier("name"), 
                             Keyword("FROM"), Identifier("users")]
```
- **Implementation**: `Tokenize` trait converts SQL strings into `SQLToken` enums
- **Learn**: Lexical analysis - breaking text into meaningful symbols

#### **Abstract Syntax Tree (AST)** (`src/command/sql/parser/sql_statement.rs`)
- Tokens are parsed into structured statement types:
  - `SelectStatement` - Represents SELECT queries with columns, table name, WHERE clauses
  - `CreateTableStatement` - Represents CREATE TABLE with column definitions
- **Implementation**: `from_tokens()` methods build AST from token streams
- **Learn**: How parsers build hierarchical structures from flat token sequences

#### **Query Execution** (`src/command/sql/select.rs`)
- AST is traversed to execute the query against the database file
- **Implementation**: `select()` function reads pages, filters records, applies aggregations
- **Key concept**: How SQL statements map to actual file I/O operations

## Repository Structure

```
src/
├── main.rs                   # CLI entry point
├── command/
│   └── sql/
│       ├── select.rs         # Query execution logic
│       └── parser/
│           ├── sql_token.rs  # Tokenization (lexing)
│           └── sql_statement.rs # AST definitions
├── models/
│   └── dbfile/
│       ├── dbfile.rs         # Main database file interface
│       ├── schema/           # Header and schema parsing
│       └── dbtable/
│           ├── table.rs      # Table interface (DBTable)
│           ├── tablerow.rs   # Row representation with String values
│           └── tablepage/    # B-tree page implementations
│               ├── leaftablepage.rs
│               ├── interiortablepage.rs
│               └── record.rs # Raw record with byte values
└── utils/
    └── varint.rs            # Varint encoding/decoding

tests/                       # Integration tests with sample databases
```

## Quick Start

## Quick Start

### Installation

Requires Rust 1.80 or later:

```bash
cargo build --release
```

### Usage Examples

```bash
# Get database metadata
cargo run -- sample.db .dbinfo

# List all tables
cargo run -- sample.db .tables

# Execute SQL queries
cargo run -- sample.db "SELECT name, color FROM apples"
cargo run -- sample.db "SELECT COUNT(*) FROM users"
```

## Learning Path

If you're using this project to learn, here's a suggested exploration order:

1. **Start with Varints** (`src/utils/varint.rs`)
   - Simple but crucial: understand variable-length integer encoding
   - Used throughout SQLite for compact storage

2. **Understand the Schema** (`src/models/dbfile/schema/`)
   - Read `schemaraw.rs` to see how the database header is parsed
   - Learn about page size, file format version, and schema storage

3. **Explore Page Types** (`src/models/dbfile/dbtable/tablepage/`)
   - `leaftablepage.rs` - Where actual data lives
   - `interiortablepage.rs` - How B-trees enable efficient lookups
   - Notice how both implement the `Table` trait

4. **Study SQL Tokenization** (`src/command/sql/parser/sql_token.rs`)
   - See how SQL text becomes tokens
   - Experiment with `"SELECT * FROM users".tokenize()`

5. **Build the AST** (`src/command/sql/parser/sql_statement.rs`)
   - Understand how tokens become structured `SelectStatement`
   - See how column names, table names, and WHERE clauses are extracted

6. **Follow Query Execution** (`src/command/sql/select.rs`)
   - Trace how a SELECT statement reads pages and produces results
   - See the connection between AST and actual file operations

## What This Project Teaches

- ✅ Binary file format parsing and data serialization
- ✅ B-tree data structures for database storage
- ✅ Lexical analysis (tokenization) for SQL
- ✅ Parser implementation and AST construction
- ✅ How SQL queries map to disk I/O operations
- ✅ Space-efficient integer encoding (varints)
- ✅ Type system design (serial types to Rust types)

## What This Project Does NOT Cover

- ❌ Write operations (INSERT, UPDATE, DELETE)
- ❌ Transactions and ACID properties
- ❌ Query optimization
- ❌ Indexes (B+ trees for indexes)
- ❌ Concurrent access / locking
- ❌ Advanced SQL features (JOINs, subqueries, etc.)

This is intentional — the goal is to deeply understand core concepts, not build a complete database.

## Build

## Build & Test

You need a recent Rust toolchain (stable channel, Rust >= 1.80). To build the project:

```bash
cargo build --release
```

To run the binary from the workspace (debug build):

```bash
cargo run -- <database_file> <command>
# example: cargo run -- ./tests/assets/sample.db .dbinfo
```

Alternatively, call the library from another binary — `src/main.rs` calls `sqlite_fsr::run`.

### Testing

Run the test suite with:

```bash
cargo test
```

The tests use real SQLite database files in `tests/assets/` including:
- `sample.db` - Simple tables for basic operations
- `superheroes.db` - Larger dataset for performance testing
- `companies.db` - Complex schema examples

Tests cover:
- Binary format parsing (page headers, cell pointers, varint decoding)
- SQL tokenization and AST construction
- Query execution and result formatting
- Edge cases (empty values, NULL handling, non-UTF8 data)

## Resources for Learning

To deepen your understanding while exploring this codebase:

- **SQLite File Format Documentation**: https://www.sqlite.org/fileformat.html
  - Official spec that this implementation follows
- **B-Tree Visualization**: https://www.cs.usfca.edu/~galles/visualization/BTree.html
  - Understand how interior/leaf pages work together
- **Crafting Interpreters** by Bob Nystrom
  - Excellent resource for understanding tokenization and parsing concepts
- **Database Internals** by Alex Petrov
  - Deep dive into storage engines and indexing

