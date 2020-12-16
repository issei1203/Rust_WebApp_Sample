#![feature(proc_macro_hygiene, decl_macro)]

mod model;

#[macro_use] extern crate rocket;

use crate::model::DataBaseConnector;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
fn hello() -> &'static str{
    "hello"
}

#[get("/index")]
fn index() -> Template{
    let mut context = HashMap::new();
    context.insert("name","issei");
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


