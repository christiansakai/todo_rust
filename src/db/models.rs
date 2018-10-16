use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use super::schema::todos;
use super::schema::todos::dsl;

#[derive(Insertable)]
#[table_name="todos"]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub description: &'a str,
}

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
}

impl<'a> NewTodo<'a> {
    pub fn insert(new_todo: NewTodo, db_conn: &PgConnection) -> Todo {
        diesel::insert_into(todos::table)
            .values(&new_todo)
            .get_result(db_conn)
            .expect("Error saving new todo")
    }
}

impl Todo {
    pub fn all(db_conn: &PgConnection) -> Vec<Todo> {
    }
}
