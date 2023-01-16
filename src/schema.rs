// @generated automatically by Diesel CLI.

diesel::table! {
    registered_servers (id) {
        id -> Unsigned<Bigint>,
        server -> Varchar,
        password -> Varchar,
        apikey -> Varchar,
    }
}
