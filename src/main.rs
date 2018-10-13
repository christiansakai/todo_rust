#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;

#[macro_use] extern crate serde_derive;
extern crate serde;

use std::collections::HashMap;
use rocket_contrib::Template;

mod static_files;

#[get("/")]
fn index() -> Template {
    Template::render("index", HashMap::<String, String>::new())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
               index,
               static_files::all,
        ])
        .attach(Template::fairing())
        .launch();
}
