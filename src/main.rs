#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
mod model;

#[post("/regist")]
fn regist(){
}

fn main(){
    rocket::ignite().mount("/",routes![regist]).launch();
}


