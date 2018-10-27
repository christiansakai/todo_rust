use std::collections::HashMap;
use rocket_contrib::Template;
use rocket::response::Redirect;

use self::super::super::db;
use self::super::super::db::models;

#[get("/")]
fn all(conn: db::DbConn) -> Template {
    models::Todo::all(&conn);
    Template::render("todo/list", HashMap::<String, String>::new())
}

#[get("/new")]
fn new() -> Template {
    Template::render("todo/new", HashMap::<String, String>::new())
}

#[get("/<id>")]
fn show(id: usize) -> Template {
    Template::render("todo/detail", HashMap::<String, String>::new())
}

#[post("/")]
fn create() -> Redirect {
    Redirect::to("/todo")
}

#[put("/<id>")]
fn toggle(id: usize) -> Redirect {
    Redirect::to("/todo/<id>")
}

#[delete("/<id>")]
fn delete(id: usize) -> Redirect {
    Redirect::to("/todo")
}
