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
        body -> VarChar,
        completed -> Bool, 
    }
}

table! {
    testing (id) {
        id -> Int4,
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
    pub body: String,
    pub completed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub user_id: &'a i32,
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = testing)]
pub struct Testing {
    pub id: i32,
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
            body: "This is my first task".to_string(),
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
            body: "some_description".to_string(),
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
            body: "take the dogs on their afternoon walk".to_string(),
            completed: false,
        },
        Task{
            id:15,
            user_id: 164,
            title: "make lunch".to_string(),
            body: "heat up more ribs".to_string(),
            completed: true,
        }
    ])
}

#[post("/", data = "<task>")]
async fn create_task(connection: Db, task: Json<Task>) -> Json<Task> {
    connection
    .run(move |c| {
        diesel::insert_into(tasks::table)
            .values(&task.into_inner())
            .get_result(c)
    })
    .await
    .map(Json)
    .expect("sigggghhhh")
}

#[get("/puppy")]
fn get_testing_puppy() -> Json<Testing> {
    Json(
        Testing {
            id: 12,
        }
    )
}

pub fn create_task(conn: &mut PgConnection, user_id: &i32, title: &str, body: &str) -> Post {
    // use crate::schema::posts;

    let new_task = NewTask { user_id, title, body };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .returning(Task::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

#[post("/", data = "<testing>")]
async fn create_test(connection: Db, testing: Json<Testing>) -> Json<Testing> {
    connection
    .run(move |c| {
        diesel::insert_into(testing::table)
        .values(&testing.into_inner())
        .get_result(c)
    })
    .await
    .map(Json)
    .expect("OY OY OY")
}


#[launch]
fn rocket() -> _ {
    let rocket= rocket::build();
    
    rocket
    .attach(Db::fairing())
    .attach(AdHoc::config::<Config>())
      .mount("/", routes![get_all_tasks, create_task, get_config])
      .mount("/tasks", routes![get_random_task, get_task])
      .mount("/testing", routes![create_test, get_testing_puppy])
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

