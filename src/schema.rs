diesel::table! {
    bird (id) {
    id -> Integer,
    name -> Varchar,
    scientific_name -> Varchar,
    commonwealth_status -> Varchar,
    }
}

diesel::table! {
    bird_sighting (id) {
    id -> Integer,
    bird_id -> Integer,
    sighting_date -> Timestamp,
    sighting_location -> Varchar,
    additional_information -> Text,
    }
}