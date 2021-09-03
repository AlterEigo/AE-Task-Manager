//!
//! TaskManager's root
//!

use gio::prelude::*;
use gtk::prelude::*;

use crate::prelude::*;
use crate::root::{RootView};

pub struct Application {
    gtk_app: gtk::Application,
}

impl Application {
    pub fn new() -> Self
    {
        Application {
            gtk_app: gtk::Application::builder()
                .application_id("org.altereigo.ae-task-manager")
                .build(),
        }
    }

    pub fn run(&self) -> i32 {
        // let root_widget: gtk::Widget = (*self.root).clone();
        Application::load_resources();
        let window = gtk::ApplicationWindow::builder()
            .title("AE Task Manager")
            .build();
        window.set_child(Some(&RootView::assemble()));
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
