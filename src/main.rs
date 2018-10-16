#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use rocket_contrib::Template;
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

mod db;
mod routes;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let conn_pool = db::init_pool(&database_url);

    rocket::ignite()
        .manage(conn_pool)
        .mount("/", routes![routes::index::index])
        .mount("/static", routes![routes::static_files::all])
        .mount("/todo", routes![
               routes::todo::all,
               routes::todo::new,
               routes::todo::show,
        ])
        .mount("/authenticate", routes![
               routes::authenticate::render_signup,
               routes::authenticate::render_login,
               routes::authenticate::signup,
               routes::authenticate::login,
        ])
        .attach(Template::fairing())
        .launch();
}
