Setting up my first Rust project using Diesel and Rocket.

// create project
cargo new task_project --bin
cd task_project

// Add rocket
cargo add rocket@0.5.0-rc.2 --features=json

// set up rocket in main.rs, but using services folder instead

    #[macro_use] extern crate rocket;

    #[get("/")]
    fn index() -> &'static str {
        "Hello, world!"
    }

    #[launch]
    fn rocket() -> _ {
        let rocket = rocket::build();           // building rocket
        rocket.mount("/", routes![index])       // mounting rocket
    }

// add serde

cargo add serde@1.0.140

// created a struct in models.rs

    #[derive(Serialize, Deserialize)]
    struct BlogPost {
        id: i32,
        title: String,
        body: String,
        published: bool,
    }

// create a GET endpoint

    #[get("/random")]
    fn get_random_blog_post() -> Json<BlogPost> {
        Json(
            BlogPost {
                id: 1,
                title: "My first post".to_string(),
                body: "This is my first post".to_string(),
                published: true,
            }
        )
    }

// mount to the task-list vector of routes in main.rs

    #[launch]
    fn rocket() -> _ {
        let rocket= rocket::build();
        
        rocket
        .mount("/", routes![index])
        .mount("/blog-posts", routes![get_random_blog_post])
    }

// add Rocket.toml file in root

    [default]
    name = "raimi"
    age = 20

// Define this config as a Rust struct in src/main.rs. Here, we name the struct as Config.

    #[derive(Deserialize)]
    struct Config {
        name: String,
        age: u8,
    }

// In Rocket, configurations are Managed States, which you can retrieve via &State<T>. Here, we are trying to use the Config values in a GET endpoint by using &State<Config>.

// pull in state from Rocket

    use rocket::State;

// add route to test grabbing config state

    #[get("/config")]
    fn get_config(config: &State<Config>) -> String {
        format!(
        "Hello, {}! You are {} years old.", config.name, config.age
    )
    }

// tell Rocket that it has to extract the Config configuration values from Rocket.toml (or another provider) and use it as a managed state. In Line 9, we include the new get_config route.

    .attach(AdHoc::config::<Config>())


// add the docker-compose.yml file

    version: '3.1'
    services:
    db:
        image: postgres
        restart: always
        environment:
        POSTGRES_USER: postgres
        POSTGRES_PASSWORD: example
        ports:
        - "5432:5432"

// add Diesel CLI and Diesel as a dependency

    cargo add diesel_cli@1.4.1 --no-default-features --features postgres   
    cargo add diesel@1.4.1

// change the blog post struct withe following so there's a table! macro for Diesel

#[macro_use] extern crate diesel;
use diesel::{Insertable, Queryable, table};

    table! {
        blog_posts (id) {
            id -> Int4,
            title -> Varchar,
            body -> Text,
            published -> Bool,
        }
    }

    #[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
    #[table_name = "blog_posts"]
    struct BlogPost {
        id: i32,
        title: String,
        body: String,
        published: bool,
    }

// the table! macro expands to a bunch of code based on the database schema to represent the blog_post table and its columns. This code is used to associate the BlogPost struct with the blog_posts table and to validate the names of the struct members against the columns id, title, body and published columns.

// Add the Rocket library that provides support for synchronous ORM.

    cargo add rocket_sync_db_pools@0.1.0-rc.2 \
      --features=diesel_postgres_pool

// configure databse in rocket.toml

    [default.databases.my_db]
    url = "postgres://postgres:example@localhost:5432/postgres"


// Tie in this configuration in our application:
    #[database("my_db")]
    pub struct Db(diesel::PgConnection);

// Then attach Db as a fairing to Rocket in Line 6:

    .attach(Db::fairing())

// pull in sync db pools

    use rocket_sync_db_pools::database;