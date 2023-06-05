// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        title -> Varchar,
        description -> Text,
        completed -> Bool,
    }
}
