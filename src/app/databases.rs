use crate::prelude::*;
use crate::app::services::DbService;

pub struct MainDb;
impl MainDb {
    fn create_new_db(name: &str, flags: sqlite::OpenFlags) -> Result<sqlite::Connection> {
        let connection = sqlite::Connection::open_with_flags(name, flags.set_create())?;
        connection.execute(
            "
            CREATE TABLE users (
                user_id varchar(255) UNIQUE PRIMARY KEY,
                first_name varchar(255),
                last_name varchar(255),
                email varchar(255),
                username varchar(255) UNIQUE NOT NULL,
                password varchar(255) NOT NULL,
                salt varchar(255) UNIQUE NOT NULL
            );
        ",
        )?;
        Ok(connection)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl DbService for MainDb {
    fn connection(&self) -> Result<sqlite::Connection> {
        let flags = sqlite::OpenFlags::new().set_read_write().set_full_mutex();
        let dbname = "appdb.sqlite";

        let conn = match sqlite::Connection::open_with_flags(&dbname, flags.clone()) {
            Ok(conn) => Ok(conn),
            _ => MainDb::create_new_db(&dbname, flags),
        };
        conn
    }
}
