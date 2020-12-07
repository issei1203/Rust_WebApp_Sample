use rusqlite::{params, Connection};

// struct Date{
//     year :usize,
//     month :usize,
//     day :usize
// }
//
// struct TodoData{
//     id :usize,
//     date :Date,
//     detail :String
// }

struct DataBaseConnector{
    path : String
}
impl DataBaseConnector{
    pub fn open(&self) -> Result<Connection,String>{
        let connection = Connection::open(&self.path);
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
}

#[cfg(test)]
mod test{
    use crate::model::DataBaseConnector;

    #[test]
    fn test_connect_database(){
        let connection_base = DataBaseConnector{ path: String::from("./test_db.db3") };
        let Connector = connection_base.open();
        debug_assert!(Connector.is_ok());
    }
}