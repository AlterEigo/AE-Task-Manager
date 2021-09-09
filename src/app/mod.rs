//!
//! TaskManager's root
//!

use gio::prelude::*;
use gtk::prelude::*;

use crate::prelude::*;
use crate::root::RootView;
use std::rc::Rc;

pub mod forms;
pub mod models;
pub mod services;

use forms::*;
use models::*;
use services::*;

pub struct Application {
    gtk_app: gtk::Application,
    db_service: Option<Rc<dyn DbService>>,
    user_service: Option<Rc<dyn UserService>>,
}

impl Application {
    pub fn new() -> Self {
        Application {
            gtk_app: gtk::Application::builder()
                .application_id("org.altereigo.ae-task-manager")
                .build(),
            db_service: None,
            user_service: None,
        }
    }

    pub fn database(self, value: &Rc<dyn DbService>) -> Self {
        Application {
            db_service: Some(value.clone()),
            ..self
        }
    }

    pub fn user_service(self, value: &Rc<dyn UserService>) -> Self {
        Application {
            user_service: Some(value.clone()),
            ..self
        }
    }

    pub fn run(&self) -> i32 {
        Application::load_resources();
        let window = gtk::ApplicationWindow::builder()
            .title("AE Task Manager")
            .build();
        let mut root = RootView::new();
        if let Some(srv) = self.user_service.as_ref() {
            root = root.user_service(Rc::clone(&srv));
        }
        let root = root.assemble();
        window.set_child(Some(&root));
        let css_provider = gtk::CssProvider::new();
        css_provider.load_from_resource("/org/altereigo/ae-task-manager/style.css");
        gtk::StyleContext::add_provider_for_screen(
            &window.screen().unwrap(),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        self.gtk_app.connect_activate(move |app| {
            window.set_application(Some(app));
            window.present();
        });
        self.gtk_app.run()
    }

    fn load_resources() {
        let bytes = include_bytes!("../../resources/resources.gresource");
        let resource_data = glib::Bytes::from(&bytes[..]);
        let res = gio::Resource::from_data(&resource_data).unwrap();
        gio::resources_register(&res);
    }
}

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

pub struct UserManager {
    db: Option<Rc<dyn DbService>>,
}

impl UserManager {
    pub fn new() -> Self {
        UserManager { db: None }
    }

    pub fn database(self, val: &Rc<dyn DbService>) -> Self {
        UserManager {
            db: Some(val.clone()),
            ..self
        }
    }

    fn unique_uid(db_conn: &sqlite::Connection) -> String {
        let mut stmt = db_conn
            .prepare(
                "
                SELECT COUNT(user_id)
                FROM users
                WHERE user_id = :uid",
            )
            .expect("Could not prepare DB statement.");
        loop {
            let uid = nanoid::nanoid!();
            stmt.reset().unwrap();
            stmt.bind_by_name(":uid", uid.as_str())
                .expect("Unsuccessful statement parameter binding.");
            if let sqlite::State::Done = stmt.next().unwrap() {
                break uid;
            }
        }
    }

    fn unique_salt(db_conn: &sqlite::Connection) -> String {
        let mut stmt = db_conn
            .prepare(
                "
                SELECT COUNT(user_id)
                FROM users
                WHERE salt = :salt",
            )
            .expect("Could not prepare DB statement.");
        loop {
            let salt = nanoid::nanoid!();
            stmt.reset().unwrap();
            stmt.bind_by_name(":salt", salt.as_str())
                .expect("Unsuccessful statement parameter binding.");
            if let sqlite::State::Done = stmt.next().unwrap() {
                break salt;
            }
        }
    }

    fn register_user(form: SignUpForm, db_conn: &sqlite::Connection) -> Result<User> {
        let mut cursor = db_conn
            .prepare(
                "
                INSERT INTO users
                VALUES (:uid, :fname, :lname, :email, :uname, :password, :salt);",
            )
            .expect("Could not prepare DB statement.")
            .into_cursor();
        let new_uid = Self::unique_uid(db_conn);
        let salt = Self::unique_salt(db_conn);
        cursor
            .bind_by_name(vec![
                (":uid", sqlite::Value::String(new_uid)),
                (":fname", sqlite::Value::String(form.first_name)),
                (":lname", sqlite::Value::String(form.last_name)),
                (":email", sqlite::Value::String(form.email)),
                (":uname", sqlite::Value::String(form.username)),
                (":password", sqlite::Value::String(form.password)),
                (":salt", sqlite::Value::String(salt)),
            ])
            .expect("Could not bind form values.");
        Err(Error::NotImplemented)
    }
}

impl UserService for UserManager {
    fn authenticate(&self, u: String, p: String) -> Result<SessionId> {
        Err(Error::NotImplemented)
    }

    fn info(&self, id: SessionId) -> Result<User> {
        Err(Error::NotImplemented)
    }

    fn sign_up(&self) -> Result<SignUpForm> {
        if let Some(db) = &self.db {
            let conn = db.connection();
            let action =
                move |form: SignUpForm| -> Result<User> { UserManager::register_user(form, conn) };
            Ok(SignUpForm::new(action))
        } else {
            Err(Error::DatabaseError)
        }
    }
}
