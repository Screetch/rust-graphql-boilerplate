
# Rust GraphQL Boilerplate


### Description

A simple Rust GraphQL Postgres API Boilerplate


## Setup
### Environment Variables

To run this project, you will need to add the following environment variables to your .env file

`DATABASE_URL`

e.g 

```
❯ echo DATABASE_URL=postgres://localhost/rust_graphql_boilerplate > .env
```
### Migration

```
❯ cargo install diesel_cli --no-default-features --features postgres
```
```
❯ diesel setup
Creating migrations directory at: /Users/rust-graphql-boilerplate/migrations
Creating database: rust-graphql-boilerplate
```
```
❯ diesel migration generate create_datas
Creating migrations/2023-01-31-131946_create_datas/up.sql
Creating migrations/2023-01-31-131946_create_datas/down.sql
```
```
❯ mv sql/up.sql sql/down.sql migrations/2023-01-31-131946_create_datas
```
```
❯ diesel migration run
Running migration 2023-01-31-131946_create_datas
```
## Running

```
❯ cargo run
```

GraphiQL available at [http://localhost:8080/graphiql](http://localhost:8080/graphiql)