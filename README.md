sudo ln -s /opt/homebrew/opt/mysql@8.0/lib/libmysqlclient.21.dylib /usr/local/mysql/lib/libmysqlclient.dylib

RUSTFLAGS="-L/opt/homebrew/opt/mysql-client@8.0/lib" cargo install diesel_cli

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