// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        description -> Text,
        completed -> Bool,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(todos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
