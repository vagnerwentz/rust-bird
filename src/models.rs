use diesel::{Insertable, Selectable};
use diesel::prelude::Queryable;
use rocket::serde::{Deserialize, Serialize};
use schema::bird;
use schema::bird_sighting;
use crate::schema;

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Bird {
    pub id: i32,
    pub name: String,
    pub scientific_name: String,
    pub commonwealth_status: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = bird)]
pub struct InputBird {
    pub name: String,
    pub scientific_name: String,
    pub commonwealth_status: String,
}

#[derive(Serialize, Queryable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = bird_sighting)]
pub struct BirdSighting {
    pub id: i32,
    pub bird_id: i32,
    pub sighting_date: chrono::NaiveDateTime,
    pub sighting_location: String,
    pub additional_information: String,
}


#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = bird_sighting)]
pub struct BirdSightingInput {
    pub bird_id: i32,
    pub sighting_location: String,
    pub additional_information: String,
    pub sighting_date: chrono::NaiveDateTime,
}