## Rusty Rocket
A basic Rocket REST API written in Rust that 
*  demonstrates how basic CRUD with Diesel ORM works
*  implements basic Authentication
*  serializes and deserializes Rust structs with Serde

### Diesel CLI
To create a DB:
```bash
diesel setup --database-url ./<name_of_DB>
```
To generate migration:
```bash
diesel migration generate <name_of_DB>
```
To list migrations:
```bash
diesel migration list --database-url=<name_of_DB>
```
To revert migrations:
```bash
diesel migration revert --database-url=<name_of_DB>
```
To run migrations:
```bash
diesel migration run --database-url=<name_of_DB>
```
<small>Running migrations will also generate a macro in `src` responsible for the Object Relational Mapping<small>