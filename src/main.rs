#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/hoge")]
fn hoge() -> String{
    format!("{}","Hello")
}
fn main() {
    rocket::ignite().mount("/",routes![hoge]).launch();
}
