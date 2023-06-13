// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        title -> Varchar,
        body -> Text,
        completed -> Bool,
    }
}

diesel::table! {
    testing (id) {
        id -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    tasks,
    testing,
);
