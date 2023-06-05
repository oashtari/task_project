#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod services;
// mod models;
use diesel::{prelude::*, table, Insertable, Queryable};
use rocket::{fairing::AdHoc, serde::json::Json, State};
use rocket_sync_db_pools::database;
use serde::{Serialize, Deserialize};

table! {
    tasks (id) {
        id -> Int4,
        user_id -> Int4, 
        title -> VarChar,
        description -> Text,
        completed -> Bool, 
    }
}

#[database("my_db")]
pub struct Db(diesel::PgConnection);

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = tasks)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Deserialize)]
struct Config {
    name: String,
    age: u8,
}

#[get("/config")]
fn get_config(config: &State<Config>) -> String {
    format!(
      "Hello, {}! You are {} years old.", config.name, config.age
   )
}

#[get("/random")]
fn get_random_task() -> Json<Task> {
    Json(
        Task {
            id: 1,
            user_id: 1,
            title: "My first task".to_string(),
            description: "This is my first task".to_string(),
            completed: false,
        }
    )
}

#[get("/<id>")]
fn get_task(id: i32) -> Json<Task> {
    Json(
        Task{
            id,
            user_id: 12,
            title: "some_title".to_string(),
            description: "some_description".to_string(),
            completed: false,
        }
    )
}

#[get("/")]
fn get_all_tasks() -> Json<Vec<Task>> {
    Json(vec![
        Task
        {
            id:14,
            user_id: 13,
            title: "walk dogs".to_string(),
            description: "take the dogs on their afternoon walk".to_string(),
            completed: false,
        },
        Task{
            id:15,
            user_id: 164,
            title: "make lunch".to_string(),
            description: "heat up more ribs".to_string(),
            completed: true,
        }
    ])
}

#[post("/", data = "<task>")]
fn create_task(task: Json<Task>) -> Json<Task> {
    task
}


#[launch]
fn rocket() -> _ {
    let rocket= rocket::build();
    
    rocket
    .attach(Db::fairing())
    .attach(AdHoc::config::<Config>())
      .mount("/", routes![get_all_tasks, create_task, get_config])
      .mount("/tasks", routes![get_random_task, get_task])
}

// #[post("/post", format="json", data="<task>" )]
// pub fn create_task(task: Json<NewTask>) -> Result<Created<Json<NewTask>>> {
//     use self::schema::tasks::dsl::*;
//     use models::Task;
//     let connection = &mut establish_connection_pg();

//     let new_task = Task {
//         id: 1, 
//         user_id: 1, 
//         title: post.title.to_string(),
//         description: post.description.to_string(),
//         completed: false,
//     };

//     diesel::insert_into(self::schema::tasks::dsl::tasks)
//         .values(&new_task)
//         .execute(connection)
//         .expect("Error saving new task.");

//     Ok(Created::new("/").body(task))

// }

    // pub id: i32,
    // pub user_id: i32,
    // pub title: String,
    // pub description: String,
    // pub completed: bool,

// #[get("/tasks")]
// fn get_all_tasks() -> Json<Vec<Task>> {
//     use self::models::Task;

//     let connection = &mut establish_connection_pg();

//     let results = 
// }

