#![feature(proc_macro_hygiene, decl_macro)]

mod model;
mod todo_type;

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use crate::model::DataBaseConnector;

#[get("/")]
fn hello() -> &'static str{
    "hello"
}

#[get("/index")]
fn index() -> Template{
    let mut context:HashMap<String,String> = HashMap::new();
    Template::render("index",context)
}

#[post("/regist", data = "<todo>")]
fn register(todo: String) -> &'static str{
    println!("{}",todo);
    "hello"
}

fn main(){
    let connection_base = DataBaseConnector{ path: String::from("./test_db.db3") };
    connection_base.create_table();

    rocket::ignite().mount("/",routes![register, hello, index]).attach(Template::fairing()).launch();
}


