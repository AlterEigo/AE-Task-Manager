//!
//! TaskManager's root
//!

use gio::prelude::*;
use gtk::prelude::*;

pub struct Application {
    gtk_app: gtk::Application,
}

impl Application {
    pub fn new() -> Self {
        let instance = Application {
            gtk_app: gtk::Application::builder()
                .application_id("Æ TaskManager")
                .build()
        };
        instance.gtk_app.connect_activate(|app| {
            let window = gtk::ApplicationWindow::builder()
                .application(app)
                .default_width(800)
                .default_height(600)
                .title("AE Task Manager")
                .build();

            window.show();
        });
        instance
    }

    pub fn run(&self) -> i32 {
        self.gtk_app.run()
    }

    fn load_resources(&self) {
        let bytes = include_bytes!("../../resources/resources.xml");
        let resource_data = glib::Bytes::from(&bytes[..]);
        let res = gio::Resource::from_data(&resource_data).unwrap();
        gio::resources_register(&res);
    }
}
