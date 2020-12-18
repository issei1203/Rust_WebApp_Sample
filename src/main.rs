#![feature(proc_macro_hygiene, decl_macro)]

mod model;
mod todo_type;

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use crate::model::{DataBaseConnector, TodoData, Date};
use crate::todo_type::TodoDataType;

const DATABASE_PATH: &str = "./test_db.db3";

#[get("/")]
fn hello() -> &'static str{
    "hello"
}

#[get("/index")]
fn index() -> Template{
    let connection_base = DataBaseConnector{ path: String::from(DATABASE_PATH) };

    let mut context:HashMap<String,Vec<TodoData>> = HashMap::new();

    let mut vec_of_do=vec![];
    let mut vec_of_doing = vec![];
    let mut vec_of_done = vec![];
    match connection_base.get_vector_data_of_todo_type(TodoDataType::Do) {
        Ok(vec)=>{vec_of_do = vec;}
        Err(_)=>{}
    }
    match connection_base.get_vector_data_of_todo_type(TodoDataType::Doing) {
        Ok(vec)=>{vec_of_doing = vec;}
        Err(_)=>{}
    }
    match connection_base.get_vector_data_of_todo_type(TodoDataType::Done) {
        Ok(vec)=>{vec_of_done = vec;}
        Err(_)=>{}
    }

    context.insert("do".to_string(),vec_of_do);
    context.insert("doing".to_string(),vec_of_doing);
    context.insert("done".to_string(),vec_of_done);
    Template::render("index",context)
}

#[post("/regist", data = "<todo>")]
fn register(todo: String) -> &'static str{
    println!("{}",todo);
    "hello"
}

fn main(){
    let connection_base = DataBaseConnector{ path: String::from(DATABASE_PATH) };
    connection_base.create_table();

    //サンプル挿入
    let sample_date = Date{year: 2020, month: 11, day: 5};
    let sample_data_of_do = TodoData{id: 1, date: sample_date, detail: "study with friends".to_string()};
    connection_base.insert_data_of_do(sample_data_of_do);

    let sample_date2 = Date{year: 2020, month: 11, day: 5};
    let sample_data_of_doing = TodoData{id: 2, date: sample_date2, detail: "go to travel".to_string()};
    connection_base.insert_data_of_do(sample_data_of_doing);

    let sample_date3 = Date{year: 2020, month: 11, day: 5};
    let sample_data_of_done = TodoData{id: 3, date: sample_date3, detail: "sleep with friends".to_string()};
    connection_base.insert_data_of_do(sample_data_of_done);

    rocket::ignite().mount("/",routes![register, hello, index]).attach(Template::fairing()).launch();
}


