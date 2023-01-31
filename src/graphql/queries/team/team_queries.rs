use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::database::database;
use crate::graphql::models::team::team_model;

pub struct TeamRoot;

#[juniper::graphql_object]
impl TeamRoot {
    fn teams() -> FieldResult<Vec<team_model::Team>> {
        use crate::schema::teams::dsl;
        let mut connection = database::establish_connection();
        let results = dsl::teams
            .limit(100)
            .load::<team_model::Team>(&mut connection);
        match results {
            Ok(res) => Ok(res),
            Err(_) => Err(FieldError::new(
                "Error loading teams",
                graphql_value!({ "code": "INTERNAL_SERVER_ERROR" }),
            )),
        }
    }
}
