use juniper::{EmptySubscription, RootNode};

use crate::graphql::queries::team::team_queries::TeamRoot;
use crate::graphql::queries::user::user_queries::UserRoot;

use crate::graphql::mutation::user::user_mutations::UserMutation;

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn users(&self) -> UserRoot {
        UserRoot
    }
    fn teams(&self) -> TeamRoot {
        TeamRoot
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn users(&self) -> UserMutation {
        UserMutation
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
