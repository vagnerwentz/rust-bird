use diesel::result::Error;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use diesel::debug_query;
use diesel::prelude::*;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use crate::{database, models};
use crate::models::*;
use crate::schema::bird::dsl::bird;
use crate::schema::bird::{commonwealth_status, id};

#[get("/")]
pub fn index() -> Json<Vec<Bird>> {
    use crate::schema::bird::dsl::bird;
    let connection = &mut database::establish_connection();
    bird.load::<Bird>(connection).map(Json).expect("Erro ao listar todos os pássaros")
}

#[post("/bird", data = "<bird_input>")]
pub fn new_bird(bird_input: Json<InputBird>) -> Json<Bird> {
    use crate::schema::bird;
    let connection = &mut database::establish_connection();

    diesel::insert_into(bird::table)
        .values(bird_input.into_inner())
        .execute(connection)
        .expect("Erro ao inserir um pássaro");


    Json(bird::table
        .order(bird::id.desc())
        .first(connection).unwrap()
    )
}

#[post("/sighting", data = "<sighting>")]
pub fn new_sighting(sighting: Json<BirdSightingInput>) -> Json<BirdSighting> {
    use crate::schema::bird_sighting;
    let connection = &mut database::establish_connection();


    diesel::insert_into(bird_sighting::table)
        .values(sighting.into_inner())
        .execute(connection)
        .expect("Erro ao inserir um avistamento");


    Json(bird_sighting::table
        .order(bird_sighting::id.desc())
        .first(connection).unwrap()
    )
}

#[get("/bird/<bird_id>")]
pub fn get_bird_by_id(bird_id: i32) -> Json<Vec<Bird>> {
    use crate::schema::bird::dsl::*;
    let connection = &mut database::establish_connection();
    bird.filter(id.eq(bird_id)).load::<Bird>(connection).map(Json).expect("Erro ao buscar o pássaro pelo id")
}

#[delete("/bird/<bird_id>")]
pub fn delete_bird(bird_id: i32) -> NoContent {
    let connection = &mut database::establish_connection();
    diesel::delete(bird.filter(id.eq(bird_id))).execute(connection).expect("Não foi possível deletar o passáro");

    NoContent
}