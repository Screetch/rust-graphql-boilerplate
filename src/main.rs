use actix_web::{web::Data, App, HttpServer};
use std::{io, sync::Arc};

mod database;
mod graphql;
mod schema;

#[actix_web::main]
async fn main() -> io::Result<()> {
    //Create Juniper Schema
    let schema = Arc::new(graphql::graphql_schema::create_schema());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql::service::graphql)
            .service(graphql::service::graphiql)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
