#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
// DB Items
#[macro_use]
extern crate lazy_static;
extern crate rocket;

// pub mod models;
// pub mod schema;

mod jenkins;

mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;
use std::thread;
use std::time::Duration;

// DB Items
lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<SqliteConnection>> = establish_connection();
}

pub fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_manager = ConnectionManager::new(&database_url[..]);
    Pool::builder()
        .max_size(15)
        .build(connection_manager)
        .expect(&format!("Couldn't build connection pool for the database at {}", database_url))
}

#[get("/spec/<series>/<openstack>/<spec>")]
fn spec(series: String, openstack: String, spec: String) -> String {
    format!("Results for {}/{}-{}", spec, series, openstack)
}

#[get("/matrix/<name>")]
fn matrix_name(name: String) -> String {
    format!("Matrix for {}", name)
}

#[get("/matrix")]
fn matrices() -> String {
    "Matrices".to_string()
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    dotenv().ok();

    // Setup Jenkins updates
    // One minute
    let sleep_duration = Duration::from_millis(1000 * 60 * 1);
    let jenkins_worker = jenkins_updater(sleep_duration);

    rocket::ignite().mount("/", routes![hello, matrices, matrix_name, spec]).launch();
}

fn jenkins_updater(update_interval: Duration) -> thread::JoinHandle<()> {
    let jenkins_url = env::var("JENKINS_URL").expect("JENKINS_URL must be set");

    thread::spawn( move || loop {
        println!("Updating from Jenkins at: {}", jenkins_url);
        thread::sleep(update_interval);
    })
}
