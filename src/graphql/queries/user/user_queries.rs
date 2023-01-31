use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::database::database;
use crate::graphql::models::user::user_model;

pub struct UserRoot;

#[juniper::graphql_object]
impl UserRoot {
    fn users() -> FieldResult<Vec<user_model::User>> {
        use crate::schema::users::dsl;
        let mut connection = database::establish_connection();
        let results = dsl::users
            .limit(100)
            .load::<user_model::User>(&mut connection);
        match results {
            Ok(res) => Ok(res),
            Err(_) => Err(FieldError::new(
                "Error loading users",
                graphql_value!({ "code": "INTERNAL_SERVER_ERROR" }),
            )),
        }
    }

    fn user(id: i32) -> FieldResult<user_model::User> {
        use crate::schema::users::dsl;
        let mut connection = database::establish_connection();
        let results = dsl::users
            .filter(dsl::id.eq(id))
            .first::<user_model::User>(&mut connection);
        match results {
            Ok(res) => Ok(res),
            Err(_) => Err(FieldError::new(
                "Error loading user",
                graphql_value!({ "code": "INTERNAL_SERVER_ERROR" }),
            )),
        }
    }
}
