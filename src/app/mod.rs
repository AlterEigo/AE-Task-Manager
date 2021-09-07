//!
//! TaskManager's root
//!

use gio::prelude::*;
use gtk::prelude::*;

use crate::prelude::*;
use crate::root::RootView;

pub mod models;
pub mod services;

use models::*;
use services::*;

pub struct Application {
    gtk_app: gtk::Application,
    db_service: Option<Box<dyn DbService>>,
}

impl Application {
    pub fn new() -> Self {
        Application {
            gtk_app: gtk::Application::builder()
                .application_id("org.altereigo.ae-task-manager")
                .build(),
            db_service: match MainDb::new() {
                Ok(initialized) => Some(Box::new(initialized)),
                _ => {
                    println!("Database service could not be initialized.");
                    None
                },
            },
        }
    }

    pub fn run(&self) -> i32 {
        // let root_widget: gtk::Widget = (*self.root).clone();
        Application::load_resources();
        let window = gtk::ApplicationWindow::builder()
            .title("AE Task Manager")
            .build();
        let root = RootView::new();
        window.set_child(Some(&root.assemble()));
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

struct MainDb {
    connection: sqlite::Connection,
}

impl MainDb {
    fn create_new_db(name: &str, flags: sqlite::OpenFlags) -> sqlite::Result<sqlite::Connection> {
        let connection = sqlite::Connection::open_with_flags(name, flags.set_create())?;
        connection.execute("
            CREATE TABLE users (
                user_id int,
                first_name varchar(255),
                last_name varchar(255),
                email varchar(255),
                username varchar(255),
                password varchar(255),
                salt varchar(255)
            );
        ")?;
        Ok(connection)
    }

    fn new() -> sqlite::Result<MainDb> {
        let flags = sqlite::OpenFlags::new()
            .set_read_write()
            .set_full_mutex();
        let dbname = "appdb.sqlite";

        let conn = match sqlite::Connection::open_with_flags(&dbname, flags.clone()) {
            Ok(conn) => Ok(conn),
            _ => MainDb::create_new_db(&dbname, flags)
        };
        match conn {
            Ok(conn) => Ok(MainDb { connection: conn }),
            Err(error) => {
                println!("Could not initialize connection.");
                println!("Reason : {}", error.message.clone().unwrap());
                Err(error)
            },
        }
    }
}

impl DbService for MainDb {
    fn connection(&self) -> &sqlite::Connection {
        &self.connection
    }
}
