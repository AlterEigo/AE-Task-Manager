//!
//! TaskManager's root
//!

use gio::prelude::*;
use gtk::prelude::*;

mod root;

use crate::prelude::*;
use root::RootView;

pub struct Application {
    gtk_app: Box<gtk::Application>,
    root: Box<dyn View>,
}

impl Application {
    pub fn new() -> Self {
        Application {
            gtk_app: Box::new(
                gtk::Application::builder()
                    .application_id("org.altereigo.tmanager")
                    .build(),
            ),
            root: Box::new(RootView::new()),
        }
    }

    pub fn run(&self) -> i32 {
        self.load_resources();
        let window = gtk::ApplicationWindow::builder()
            .application(&*self.gtk_app)
            .default_width(800)
            .default_height(600)
            .title("AE Task Manager")
            .build();
        window.set_child(Some(&*self.root.assemble()));
        self.gtk_app.connect_activate(move |_app| window.present());
        self.gtk_app.run()
    }

    fn load_resources(&self) {
        let bytes = include_bytes!("../../resources/resources.gresource");
        let resource_data = glib::Bytes::from(&bytes[..]);
        let res = gio::Resource::from_data(&resource_data).unwrap();
        gio::resources_register(&res);
    }
}
