#![feature(proc_macro_hygiene, decl_macro)]

mod model;
mod todo_type;

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use crate::model::{DataBaseConnector, TodoData, Date};
use crate::todo_type::TodoDataType;
use rocket::request::{Form};

const DATABASE_PATH: &str = "./test_db.db3";

#[derive(FromForm, Clone)]
struct InputTodo {
    id: String,
    year: String,
    month: String,
    day: String,
    detail: String
}
#[derive(FromForm, Clone)]
struct UpdateTodo{
    id: String,
    flag: String
}

#[get("/")]
fn hello() -> &'static str{
    "hello"
}

#[post("/regist", data="<input_todo>")]
fn regist(input_todo: Form<InputTodo>) -> Template{
    let connection_base = DataBaseConnector{ path: String::from(DATABASE_PATH) };

    let id;
    let year;
    let month;
    let day;
    let detail = input_todo.detail.clone();
    match input_todo.id.parse::<i64>(){
        Ok(input)=>{id = input;}
        Err(_)=>{return create_index_template(connection_base);}
    }
    match input_todo.year.parse::<i64>(){
        Ok(input)=>{year = input;}
        Err(_)=>{return create_index_template(connection_base);}
    }
    match input_todo.month.parse::<i64>(){
        Ok(input)=>{month = input;}
        Err(_)=>{return create_index_template(connection_base);}
    }
    match input_todo.day.parse::<i64>(){
        Ok(input)=>{day = input;}
        Err(_)=>{return create_index_template(connection_base);}
    }

    let input_date = Date{year, month, day };
    let input_data = TodoData{id, date: input_date, detail };

    connection_base.insert_data_of_do(input_data);


    create_index_template(connection_base)
}

#[post("/update", data="<update_todo>")]
fn update_todo(update_todo: Form<UpdateTodo>) -> Template{
    let connection_base = DataBaseConnector{ path: String::from(DATABASE_PATH) };
    let id ;
    let flag;
    match  update_todo.id.parse::<i64>(){
        Ok(update)=>{id = update;}
        Err(_)=>{return create_index_template(connection_base);}
    }

    flag = update_todo.flag.clone();

    connection_base.update_data(id, flag);

    create_index_template(connection_base)
}

#[post("/delete", data="<delete_todo>")]
fn delete_todo(delete_todo: Form<UpdateTodo>) -> Template{
    let connection_base = DataBaseConnector{ path: String::from(DATABASE_PATH) };
    let id ;
    match  delete_todo.id.parse::<i64>(){
        Ok(delete)=>{id = delete;}
        Err(_)=>{return create_index_template(connection_base);}
    }

    connection_base.delete_data(id);

    create_index_template(connection_base)
}

#[get("/index")]
fn index() -> Template{
    let connection_base = DataBaseConnector{ path: String::from(DATABASE_PATH) };

    create_index_template(connection_base)
}

fn create_index_template(connection_base: DataBaseConnector) -> Template{
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

    rocket::ignite().mount("/",routes![regist, hello, index, update_todo, delete_todo]).attach(Template::fairing()).launch();
}


