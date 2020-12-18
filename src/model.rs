use rusqlite::{params, Connection};
use std::collections::HashMap;
use crate::todo_type::TodoDataType;
#[derive(serde::Serialize)]
pub struct Date{
    pub year :i64,
    pub month :i64,
    pub day :i64
}
#[derive(serde::Serialize)]
pub struct TodoData{
    pub id :i64,
    pub date :Date,
    pub detail :String
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

    pub fn get_data_of_todo_type(&self, data_type:TodoDataType) -> Result<HashMap<String,TodoData>,String>{
        let connection = Connection::open(&self.path);
        if connection.is_err(){
            return Err("Cannot Connect DataBase".to_string());
        }
        let manage = connection.unwrap();

        let flag;
        match data_type{
            TodoDataType::Do => {flag = "\"do\""},
            TodoDataType::Doing => {flag = "\"doing\""},
            TodoDataType::Done => {flag = "\"done\""}
        }

        let raw_select_query = format!("{}{}","select id,year,month,day,detail From list WHERE flag = ", flag);

        let mut select_query = manage.prepare(raw_select_query.as_str());
        if select_query.is_err(){
            return Err("Cannot select query".to_string());
        }
        let mut select_query_result = select_query.unwrap();

        let itr = select_query_result.query_map(params![],|row|{
            Ok(TodoData{
                id: row.get(0).unwrap(),
                date: Date{year: row.get(1).unwrap(), month: row.get(2).unwrap(), day: row.get(3).unwrap()},
                detail: row.get(4).unwrap()
            })
        });

        if itr.is_err(){
            return Err("Cannot select query2".to_string());
        }
        let itr_result = itr.unwrap();

        let mut index_map:HashMap<String, TodoData> = Default::default();

        let mut count = 0;
        for value in itr_result{
            if value.is_err(){
                return Err("Cannot select query3".to_string());
            }
            index_map.insert(count.to_string(),value.unwrap());
            count = count +1;
        }
        Ok(index_map)

    }

    pub fn get_vector_data_of_todo_type(&self, data_type:TodoDataType) -> Result<Vec<TodoData>,String>{
        let connection = Connection::open(&self.path);
        if connection.is_err(){
            return Err("Cannot Connect DataBase".to_string());
        }
        let manage = connection.unwrap();

        let flag;
        match data_type{
            TodoDataType::Do => {flag = "\"do\""},
            TodoDataType::Doing => {flag = "\"doing\""},
            TodoDataType::Done => {flag = "\"done\""}
        }

        let raw_select_query = format!("{}{}","select id,year,month,day,detail From list WHERE flag = ", flag);

        let mut select_query = manage.prepare(raw_select_query.as_str());
        if select_query.is_err(){
            return Err("Cannot select query".to_string());
        }
        let mut select_query_result = select_query.unwrap();

        let itr = select_query_result.query_map(params![],|row|{
            Ok(TodoData{
                id: row.get(0).unwrap(),
                date: Date{year: row.get(1).unwrap(), month: row.get(2).unwrap(), day: row.get(3).unwrap()},
                detail: row.get(4).unwrap()
            })
        });

        if itr.is_err(){
            return Err("Cannot select query2".to_string());
        }
        let itr_result = itr.unwrap();

        let mut index_vec = vec![];
        let mut count = 0;
        for value in itr_result{
            if value.is_err(){
                return Err("Cannot select query3".to_string());
            }
            index_vec.insert(count,value.unwrap());
            count = count +1;
        }
        Ok(index_vec)

    }
}


#[cfg(test)]
mod test{
    use crate::model::{DataBaseConnector, Date, TodoData};
    use crate::todo_type::TodoDataType;
    use std::collections::HashMap;

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

    #[test]
    fn test_get_database(){
        let connection_base = DataBaseConnector{ path: String::from("./test_db.db3") };
        connection_base.create_table();
        let sample_date = Date{year: 2020, month: 11, day: 5};
        let sample_data_of_do = TodoData{id: 1, date: sample_date, detail: "study with friends".to_string()};
        connection_base.insert_data_of_do(sample_data_of_do);

        let sample_date2 = Date{year: 2020, month: 11, day: 5};
        let sample_data_of_doing = TodoData{id: 2, date: sample_date2, detail: "go to travel".to_string()};
        connection_base.insert_data_of_do(sample_data_of_doing);

        let sample_date3 = Date{year: 2020, month: 11, day: 5};
        let sample_data_of_done = TodoData{id: 3, date: sample_date3, detail: "sleep with friends".to_string()};
        connection_base.insert_data_of_do(sample_data_of_done);

        let result_map = connection_base.get_data_of_todo_type(TodoDataType::Do);
        match result_map{
            Ok(con) => {
                assert_eq!(con.len(),3)
            }
            Err(word) => {println!("{}",word)}
        }
    }
    #[test]
    fn test_get_vec_data_database(){
        let connection_base = DataBaseConnector{ path: String::from("./test_db.db3") };
        connection_base.create_table();
        let sample_date = Date{year: 2020, month: 11, day: 5};
        let sample_data_of_do = TodoData{id: 1, date: sample_date, detail: "study with friends".to_string()};
        connection_base.insert_data_of_do(sample_data_of_do);

        let sample_date2 = Date{year: 2020, month: 11, day: 5};
        let sample_data_of_doing = TodoData{id: 2, date: sample_date2, detail: "go to travel".to_string()};
        connection_base.insert_data_of_do(sample_data_of_doing);

        let sample_date3 = Date{year: 2020, month: 11, day: 5};
        let sample_data_of_done = TodoData{id: 3, date: sample_date3, detail: "sleep with friends".to_string()};
        connection_base.insert_data_of_do(sample_data_of_done);

        let result_vec = connection_base.get_vector_data_of_todo_type(TodoDataType::Do);
        match result_vec{
            Ok(con) => {
                assert_eq!(con.len(),3)
            }
            Err(word) => {println!("{}",word)}
        }
    }
}