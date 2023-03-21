// @generated automatically by Diesel CLI.

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
