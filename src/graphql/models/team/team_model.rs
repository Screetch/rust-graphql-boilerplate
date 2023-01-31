use diesel::prelude::*;
use diesel::Queryable;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::database::database;
use crate::graphql::models::user::user_model;

#[derive(Queryable)]
pub struct Team {
    pub id: i32,
    pub name: String,
}

#[juniper::graphql_object(description = "A team of users")]
impl Team {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn users(&self) -> FieldResult<Vec<user_model::User>> {
        use crate::schema::users::dsl;
        let mut connection = database::establish_connection();
        let results = dsl::users
            .filter(dsl::team_id.eq(self.id))
            .limit(100)
            .load::<user_model::User>(&mut connection);
        match results {
            Ok(res) => Ok(res),
            Err(_) => Err(FieldError::new(
                "Error loading teams users",
                graphql_value!({ "code": "INTERNAL_SERVER_ERROR" }),
            )),
        }
    }
}
