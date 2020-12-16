use rusqlite::{params, Connection};

struct Date{
    year :i64,
    month :i64,
    day :i64
}

 pub struct TodoData{
    id :i64,
    date :Date,
    detail :String
}

pub struct DataBaseConnector{
    pub path : String
}
impl DataBaseConnector{
    pub fn create_table(&self) -> Result<Connection,String>{
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

    pub fn insert_data_of_do(&self, data: TodoData) -> Result<Connection,String>{
        let connection = Connection::open(&self.path);
        if connection.is_err(){
            return Err("Cannot Connect DataBase".to_string());
        }

        let manager = connection.unwrap();
        let insert_query = manager.execute(
            "INSERT INTO list VALUES($1,$2,$3,$4,?5,?6)"
                ,params![data.id, data.date.year, data.date.month, data.date.day, data.detail, "do".to_string()]
        );
        if insert_query.is_err(){
            return Err("Cannot insert data of 'do'".to_string());
        }

        Ok(manager)
    }

    pub fn update_data(&self, id: i64, flag: String) -> Result<Connection,String>{
        let connection = Connection::open(&self.path);
        if connection.is_err(){
            return Err("Cannot Connect DataBase".to_string());
        }

        let manager = connection.unwrap();
        let update_query = manager.execute("UPDATE list set flag=?1 where id=$2 "
            ,params![flag, id]
        );

        if update_query.is_err(){
            return Err("Cannot insert data".to_string());
        }

        Ok(manager)
    }
}


#[cfg(test)]
mod test{
    use crate::model::{DataBaseConnector, TodoData, Date};

    #[test]
    fn test_connect_database(){
        let connection_base = DataBaseConnector{ path: String::from("./test_db.db3") };
        let connector = connection_base.create_table();
        match connector{
            Ok(con) => {
                con.close();
                println!("successful");
            }
            Err(word) => {println!("{}",word)}
        }
    }

    #[test]
    fn test_insert_database(){
        let connection_base = DataBaseConnector{ path: String::from("./test_db.db3") };
        connection_base.create_table();
        let sample_date = Date{year: 2020, month: 11, day: 5};
        let sample_data_of_do = TodoData{id: 1, date: sample_date, detail: "study with friends".to_string()};

        let result = connection_base.insert_data_of_do(sample_data_of_do);
        match result{
            Ok(con) => {
                con.close();
                println!("successful");
            }
            Err(word) => {println!("{}",word)}
        }
    }

    #[test]
    fn test_update_database(){
        let connection_base = DataBaseConnector{ path: String::from("./test_db.db3") };
        connection_base.create_table();
        let sample_date = Date{year: 2020, month: 11, day: 5};
        let sample_data_of_do = TodoData{id: 1, date: sample_date, detail: "study with friends".to_string()};
        connection_base.insert_data_of_do(sample_data_of_do);

        let result = connection_base.update_data(1,"done".to_string());
        match result{
            Ok(con) => {
                con.close();
                println!("successful");
            }
            Err(word) => {println!("{}",word)}
        }
    }
}