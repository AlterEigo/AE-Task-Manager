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
    pub fn builder() -> ApplicationBuilder {
        Default::default()
    }

    fn assemble_root(&self) -> gtk::Widget {
        let mut view = RootView::new();
        view = view.user_service(&self.user_service);
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
