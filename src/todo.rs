use std::collections::HashMap;
use rocket_contrib::Template;

#[get("/")]
fn all() -> Template {
    Template::render("todo/list", HashMap::<String, String>::new())
}

#[get("/new")]
fn new() -> Template {
    Template::render("todo/new", HashMap::<String, String>::new())
}

#[get("/id")]
fn show() -> Template {
    Template::render("todo/detail", HashMap::<String, String>::new())
}
