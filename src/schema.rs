// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Nullable<BigInt>,
        description -> Text,
        installed_on -> Timestamp,
        success -> Bool,
        checksum -> Binary,
        execution_time -> BigInt,
    }
}

diesel::table! {
    blog_posts (id) {
        id -> Integer,
        url -> Text,
        title -> Text,
        author -> Text,
        created_date -> Text,
        description -> Text,
        content -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    blog_posts,
);
