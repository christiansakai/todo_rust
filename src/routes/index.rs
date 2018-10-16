use std::collections::HashMap;
use rocket_contrib::Template;

#[get("/")]
fn index() -> Template {
    Template::render("index", HashMap::<String, String>::new())
}
