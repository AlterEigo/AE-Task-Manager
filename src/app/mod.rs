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
                .application_id("org.altereigo.tmanager")
                .build(),
        }
    }

    pub fn run(&self) -> i32 {
        self.load_resources();
        // let root_widget: gtk::Widget = (*self.root).clone();
        self.gtk_app.connect_activate(move |app| {
            let window = gtk::ApplicationWindow::builder()
                .application(app)
                .title("AE Task Manager")
                .build();
            let root = RootView::new();
            let root = root.assemble();
            window.set_child(Some(&root));
            window.present();
        });
        self.gtk_app.run()
    }

    fn load_resources(&self) {
        let bytes = include_bytes!("../../resources/resources.gresource");
        let resource_data = glib::Bytes::from(&bytes[..]);
        let res = gio::Resource::from_data(&resource_data).unwrap();
        gio::resources_register(&res);
    }
}
