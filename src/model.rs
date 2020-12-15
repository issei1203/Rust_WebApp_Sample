use rusqlite::{params, Connection, NO_PARAMS};

struct Date{
    year :usize,
    month :usize,
    day :usize
}

struct TodoData{
    id :usize,
    date :Date,
    detail :String
}

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

        let drop_table_query = manager.execute("DROP TABLE IF EXISTS list",params![]);
        if drop_table_query.is_err(){
            return Err("Cannot Drop table of 'list'".to_string());
        }
        let create_list = manager.execute(
            "CREATE TABLE list (
                id INTEGER PRIMARY KEY,
                year INTEGER,
                month INTEGER,
                day INTEGER,
                detail TEXT NOT NULL,
                flag TEXT NOT NULL
        )",params![]);
        if create_list.is_err(){
            return Err("Cannot Create Tables".to_string());
        }

        Ok(manager)
    }

    pub fn insert(&self, data: TodoData) -> Result<String,String>{

    }
}

#[cfg(test)]
mod test{
    use crate::model::DataBaseConnector;

    #[test]
    fn test_connect_database(){
        let connection_base = DataBaseConnector{ path: String::from("./test_db.db3") };
        let connector = connection_base.open();
        match connector{
            Ok(con) => {con.close();}
            Err(word) => {println!("{}",word)}
        }
    }
}