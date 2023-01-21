// @generated automatically by Diesel CLI.

diesel::table! {
    bans (id) {
        id -> Unsigned<Bigint>,
        identifiers -> Json,
        reason -> Varchar,
        server -> Varchar,
        expires -> Unsigned<Bigint>,
    }
}

diesel::table! {
    registered_servers (id) {
        id -> Unsigned<Bigint>,
        server -> Varchar,
        password -> Varchar,
        apikey -> Varchar,
        verified -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(bans, registered_servers,);
