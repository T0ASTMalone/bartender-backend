// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    cocktails (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ingredients (id) {
        id -> Uuid,
        name -> Varchar,
        measurement -> Varchar,
        cocktail_id -> Uuid,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    instructions (id) {
        id -> Uuid,
        instruction -> Varchar,
        step -> Int2,
        cocktail_id -> Uuid,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    todos (id) {
        id -> Varchar,
        title -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        category_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(ingredients -> cocktails (cocktail_id));
diesel::joinable!(instructions -> cocktails (cocktail_id));
diesel::joinable!(todos -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    cocktails,
    ingredients,
    instructions,
    todos,
    users,
);
