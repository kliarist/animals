// @generated automatically by Diesel CLI.

diesel::table! {
    animals (id) {
        id -> Int4,
        species -> Varchar,
        common_name -> Varchar,
        habitat -> Varchar,
        lifespan -> Int4,
        is_endangered -> Bool,
    }
}
