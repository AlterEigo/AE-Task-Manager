//!
//! TaskManager's root
//!

use gio::prelude::*;
use gtk::prelude::*;

use crate::prelude::*;
use crate::root::RootView;

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
                _ => None,
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

pub struct SessionId(String);
pub struct SignUpForm(String, String);
pub struct UserModel;
pub struct TaskModel;
pub struct BoardModel;

struct MainDb {
    connection: sqlite::Connection,
}

impl MainDb {
    fn new() -> sqlite::Result<MainDb> {
        let conn = sqlite::open(":memory:");
        match conn {
            Ok(conn) => Ok(MainDb { connection: conn }),
            Err(error) => Err(error),
        }
    }
}

impl DbService for MainDb {
    fn connection(&self) -> &sqlite::Connection {
        &self.connection
    }
}

pub trait DbService {
    fn connection(&self) -> &sqlite::Connection;
}

pub trait UserService {
    fn authenticate(&self, u: String, p: String) -> Option<SessionId>;

    fn info(&self, t: SessionId) -> UserModel;

    fn register(&self, f: SignUpForm) -> Option<SessionId>;
}

pub trait TaskService {}

pub trait BoardService {}
