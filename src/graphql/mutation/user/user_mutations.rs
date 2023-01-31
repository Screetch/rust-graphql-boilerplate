use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::database::database;
use crate::graphql::models::user::user_model;

pub struct UserMutation;

#[juniper::graphql_object]
impl UserMutation {
    fn create_user(data: user_model::InsertUser) -> FieldResult<user_model::User> {
        use crate::schema::users::dsl;
        let mut connection = database::establish_connection();
        let results = diesel::insert_into(dsl::users)
            .values(&data)
            .get_result(&mut connection);
        match results {
            Ok(res) => Ok(res),
            Err(_) => Err(FieldError::new(
                "Error inserting user",
                graphql_value!({ "code": "INTERNAL_SERVER_ERROR" }),
            )),
        }
    }

    fn update_user(id: i32, data: user_model::InsertUser) -> FieldResult<user_model::User> {
        use crate::schema::users::dsl;
        let mut connection = database::establish_connection();
        let results = diesel::update(dsl::users.find(id))
            .set(&data)
            .get_result::<user_model::User>(&mut connection);
        match results {
            Ok(res) => Ok(res),
            Err(_) => Err(FieldError::new(
                "Error updating user",
                graphql_value!({ "code": "BAD_USER_INPUT" }),
            )),
        }
    }

    fn delete_user(id: i32) -> FieldResult<user_model::User> {
        use crate::schema::users::dsl;
        let mut connection = database::establish_connection();
        let results =
            diesel::delete(dsl::users.find(id)).get_result::<user_model::User>(&mut connection);
        match results {
            Ok(res) => Ok(res),
            Err(_) => Err(FieldError::new(
                "Error deleting user",
                graphql_value!({ "code": "BAD_USER_INPUT" }),
            )),
        }
    }
}
