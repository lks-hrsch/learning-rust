// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Integer,
        name -> Text,
        balance -> Integer,
        deleted -> Bool,
    }
}
