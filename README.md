# Todo Rust

A simple Todo app in Rust programming language using Rocket framework and Diesel ORM.

## Prerequisites

1. Install Rust toolchain

   `curl https://sh.rustup.rs -sSf | sh`

2. Install Rust nightly in this repository

   `cd todo_rust && rustup override set nightly`

3. Install PostgreSQL

   ```
   brew install postgresql
   brew services start postgresql`
   ```

4. Create the Database

   `createdb todo_rust`

5. Install Diesel ORM

   This has to be installed using the correct features, otherwise Diesel will complain with unclear error message.

   `cargo install diesel_cli --no-default-features --features postgres`

6. Run migration

   `diesel migration run --database-url postgres://localhost:5432/todo_rust`

## Development

To develop using live reload,

`watchexec --exts tera --restart "cargo run"`
