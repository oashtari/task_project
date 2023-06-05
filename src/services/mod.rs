extern crate diesel;
extern crate rocket;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::{get, post};
// use crate::models;
// use crate::schema;
use rocket_dyn_templates::{context, Template};
use std::env;

// pub fn establish_connection_pg() -> PgConnection {
//     dotenv.ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
//     PgConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
// }

#[derive(Serialize, Deserialize)]
pub struct NewTask {
    title: String,
    description: String,
    // completed: false,
}

