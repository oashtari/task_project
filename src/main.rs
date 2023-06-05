#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod services;
// mod models;
use rocket::{fairing::AdHoc, serde::json::Json, State};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = tasks)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
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

#[launch]
fn rocket() -> _ {
    let rocket= rocket::build();
    
    rocket
      .mount("/", routes![])
      .mount("/tasks", routes![get_random_task])
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

