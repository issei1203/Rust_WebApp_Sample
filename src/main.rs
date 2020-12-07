#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rusqlite::{params, Connection};
mod model;

#[post("/regist")]
fn regist(){
}

fn init() -> Result<Connection, String> {
    let connection = Connection::open_in_memory();
    if connection.is_err(){
        return Err("Cannot Connect DataBase".to_string());
    }
    let manager = connection.unwrap();
    manager.execute(
        "CREATE TABLE list (
                id INTEGER PRIMARY KEY,
                year INTEGER,
                month INTEGER,
                day INTEGER,
                detail TEXT NOT NULL,
                flag TEXT NOT NULL
        )",params![]);
    Ok(manager)
}

fn main() -> Result<(), ()> {
    let manager = init();
    if manager.is_err(){
        return Err(());
    }
    rocket::ignite().mount("/",routes![regist]).launch();
    Ok(())
}

#[cfg(test)]
mod test{
    use crate::init;

    #[test]
    fn test_ruqslite(){
        init();
    }
}

