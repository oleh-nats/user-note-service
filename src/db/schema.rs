// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(notes, users,);
