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
pub mod databases;

use forms::*;
use models::*;
use services::*;

pub struct Application {
    gtk_app: gtk::Application,
    db_service: Rc<dyn DbService>,
    user_service: Rc<dyn UserService>,
}

#[derive(Default,Clone)]
pub struct ApplicationBuilder {
    db: Option<Rc<dyn DbService>>,
    us: Option<Rc<dyn UserService>>
}

impl ApplicationBuilder {
    pub fn build(self) -> Result<Application> {
        if let None = &self.us {
            return Err(Error::BuilderError("Did not provide any user service."));
        }
        if let None = &self.db {
            return Err(Error::BuilderError("Did not provide any database."))
        }

        Ok(Application {
            gtk_app: gtk::Application::builder()
                .application_id("org.altereigo.ae-task-manager")
                .build(),
            db_service: self.db.unwrap(),
            user_service: self.us.unwrap(),
        })
    }

    pub fn database(self, rc: &Rc<dyn DbService>) -> Self {
        Self {
            db: Some(rc.clone()),
            ..self
        }
    }

    pub fn user_service(self, rc: &Rc<dyn UserService>) -> Self {
        Self {
            us: Some(rc.clone()),
            ..self
        }
    }
}

impl Application {
    fn assemble_root(&self) -> gtk::Widget {
        let mut view = RootView::new();
        if let Some(srv) = &self.user_service {
            view = view.user_service(&srv);
        };
        view.assemble()
    }

    pub fn run(&self) -> i32 {
        Application::load_resources();
        let window = gtk::ApplicationWindow::builder()
            .title("AE Task Manager")
            .build();
        let root = self.assemble_root();
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
