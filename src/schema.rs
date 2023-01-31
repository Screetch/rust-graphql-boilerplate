// @generated automatically by Diesel CLI.

diesel::table! {
    teams (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        team_id -> Int4,
    }
}

diesel::joinable!(users -> teams (team_id));

diesel::allow_tables_to_appear_in_same_query!(
    teams,
    users,
);
