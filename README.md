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