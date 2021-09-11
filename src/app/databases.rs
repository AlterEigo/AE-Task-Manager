use crate::prelude::*;
use crate::app::services::DbService;

pub struct MainDb {
    connection: sqlite::Connection,
}

impl MainDb {
    fn create_new_db(name: &str, flags: sqlite::OpenFlags) -> sqlite::Result<sqlite::Connection> {
        let connection = sqlite::Connection::open_with_flags(name, flags.set_create())?;
        connection.execute(
            "
            CREATE TABLE users (
                user_id varchar(255),
                first_name varchar(255),
                last_name varchar(255),
                email varchar(255),
                username varchar(255),
                password varchar(255),
                salt varchar(255)
            );
        ",
        )?;
        Ok(connection)
    }

    pub fn new() -> Result<MainDb> {
        let flags = sqlite::OpenFlags::new().set_read_write().set_full_mutex();
        let dbname = "appdb.sqlite";

        let conn = match sqlite::Connection::open_with_flags(&dbname, flags.clone()) {
            Ok(conn) => Ok(conn),
            _ => MainDb::create_new_db(&dbname, flags),
        };
        match conn {
            Ok(conn) => Ok(MainDb { connection: conn }),
            Err(error) => {
                println!("Could not initialize connection.");
                println!("Reason : {:?}", error);
                Err(Error::InitializationError)
            }
        }
    }
}

impl DbService for MainDb {
    fn connection(&self) -> &sqlite::Connection {
        &self.connection
    }
}
