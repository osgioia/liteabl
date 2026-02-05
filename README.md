# LiteABL

LiteABL is an **ABL-like** language engine built specifically for **SQLite**. It serves as a lightweight parser and wrapper designed to translate Progress OpenEdge ABL-style statements into standard SQL.

## Disclaimer

**Important**: This project is an independent Proof of Concept (PoC) created for educational purposes. It is a clean-room interpretation of ABL syntax and does NOT use any proprietary code, libraries, or documentation from Progress Software Corporation. It is NOT affiliated with, endorsed by, or connected to Progress Software Corporation, OpenEdge, or any of their subsidiaries. All trademarks belong to their respective owners.

It provides a Terminal User Interface (TUI) built with `ratatui` to browse and interact with query results.

## Features

- **ABL to SQL Translation**: Supports key ABL statements like `FOR EACH`, `FIND FIRST`, `CREATE`, and `DELETE`.
- **Complex Logic**: Handles nested expressions, logical operators (`AND`, `OR`), and a wide range of comparison operators.
- **Verbose Mode**: Optional technical insights into generated SQL and tokenization for debugging.

## Project Structure

- `src/`: Rust source code.
  - `lexer.rs`: Lexical analysis and tokenization.
  - `parser.rs`: Recursive descent parser and AST construction.
  - `sqlgen.rs`: ABL-AST to SQLite SQL translator.
  - `runtime.rs`: Execution orchestrator.
  - `tui.rs`: UI rendering module.
- `tests/`: ABL script examples for testing.
- `init.sql`: Database schema and seed data.
- `setup_db.sh`: Utility script to initialize the SQLite database.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- `sqlite3` command line tool

### Installation

Clone the repository and build the project:

```bash
cargo build
```

### Database Setup

Run the setup script to create the initial `test.db` in the root directory:

```bash
./setup_db.sh
```

### Running the Application

To run a script through the LiteABL engine:

```bash
cargo run -- test.db tests/queries.p
```

### Command Line Options

- `-v`, `--verbose`: Prints generated SQL, tokens, and affected rows count.

## Controls (TUI)

- **Enter**: Scroll to the next batch of results or continue to the next statement.
- **Ctrl+C**: Exit the application immediately.

## License

This project is licensed under the MIT License. The software is provided "AS IS", without warranty of any kind.
