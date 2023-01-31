use diesel::prelude::*;
use diesel::Queryable;

use crate::schema::users;

#[derive(juniper::GraphQLInputObject, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct InsertUser {
    pub name: String,
    pub team_id: i32,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub team_id: i32,
}

#[juniper::graphql_object(description = "A User")]
impl User {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn team_id(&self) -> i32 {
        self.team_id
    }
}
