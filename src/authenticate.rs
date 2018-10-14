use std::collections::HashMap;
use rocket_contrib::Template;
use rocket::response::Redirect;

#[get("/signup")]
fn render_signup() -> Template {
    Template::render("authenticate/signup", HashMap::<String, String>::new())
}

#[get("/login")]
fn render_login() -> Template {
    Template::render("authenticate/login", HashMap::<String, String>::new())
}

#[post("/signup")]
fn signup() -> Redirect {
    Redirect::to("/authenticate/login")
}

#[post("/login")]
fn login() -> Redirect {
    Redirect::to("/todo")
}

