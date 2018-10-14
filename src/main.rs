#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;

#[macro_use] extern crate serde_derive;
extern crate serde;

use std::collections::HashMap;
use rocket_contrib::Template;

mod static_files;
mod todo;
mod authenticate;

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
        .mount("/todo", routes![
               todo::all,
               todo::new,
               todo::show,
        ])
        .mount("/authenticate", routes![
               authenticate::signup,
               authenticate::login,
        ])
        .attach(Template::fairing())
        .launch();
}
