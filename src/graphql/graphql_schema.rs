use diesel::prelude::*;
use diesel::Queryable;
use juniper::{graphql_value, EmptySubscription, FieldError, FieldResult, RootNode};

use crate::database::database;
use crate::schema::users;

#[derive(Queryable)]
struct User {
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

    pub fn users(&self) -> FieldResult<Vec<User>> {
        use crate::schema::users::dsl;
        let mut connection = database::establish_connection();
        let results = dsl::users
            .filter(dsl::team_id.eq(self.id))
            .limit(100)
            .load::<User>(&mut connection);
        match results {
            Ok(res) => Ok(res),
            Err(_) => Err(FieldError::new(
                "Error loading teams users",
                graphql_value!({ "code": "INTERNAL_SERVER_ERROR" }),
            )),
        }
    }
}

#[derive(juniper::GraphQLInputObject, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct InsertUser {
    pub name: String,
    pub team_id: i32,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn users() -> FieldResult<Vec<User>> {
        use crate::schema::users::dsl;
        let mut connection = database::establish_connection();
        let results = dsl::users.limit(100).load::<User>(&mut connection);
        match results {
            Ok(res) => Ok(res),
            Err(_) => Err(FieldError::new(
                "Error loading users",
                graphql_value!({ "code": "INTERNAL_SERVER_ERROR" }),
            )),
        }
    }

    fn user(id: i32) -> FieldResult<User> {
        use crate::schema::users::dsl;
        let mut connection = database::establish_connection();
        let results = dsl::users
            .filter(dsl::id.eq(id))
            .first::<User>(&mut connection);
        match results {
            Ok(res) => Ok(res),
            Err(_) => Err(FieldError::new(
                "Error loading user",
                graphql_value!({ "code": "INTERNAL_SERVER_ERROR" }),
            )),
        }
    }

    fn teams() -> FieldResult<Vec<Team>> {
        use crate::schema::teams::dsl;
        let mut connection = database::establish_connection();
        let results = dsl::teams.limit(100).load::<Team>(&mut connection);
        match results {
            Ok(res) => Ok(res),
            Err(_) => Err(FieldError::new(
                "Error loading teams",
                graphql_value!({ "code": "INTERNAL_SERVER_ERROR" }),
            )),
        }
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_user(data: InsertUser) -> FieldResult<User> {
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

    fn update_user(id: i32, data: InsertUser) -> FieldResult<User> {
        use crate::schema::users::dsl;
        let mut connection = database::establish_connection();
        let results = diesel::update(dsl::users.find(id))
            .set(&data)
            .get_result::<User>(&mut connection);
        match results {
            Ok(res) => Ok(res),
            Err(_) => Err(FieldError::new(
                "Error updating user",
                graphql_value!({ "code": "BAD_USER_INPUT" }),
            )),
        }
    }

    fn delete_user(id: i32) -> FieldResult<User> {
        use crate::schema::users::dsl;
        let mut connection = database::establish_connection();
        let results = diesel::delete(dsl::users.find(id)).get_result::<User>(&mut connection);
        match results {
            Ok(res) => Ok(res),
            Err(_) => Err(FieldError::new(
                "Error deleting user",
                graphql_value!({ "code": "BAD_USER_INPUT" }),
            )),
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
