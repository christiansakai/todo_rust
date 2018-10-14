use std::collections::HashMap;
use rocket_contrib::Template;

#[get("/signup")]
fn signup() -> Template {
    Template::render("authenticate/signup", HashMap::<String, String>::new())
}

#[get("/login")]
fn login() -> Template {
    Template::render("authenticate/login", HashMap::<String, String>::new())
}
