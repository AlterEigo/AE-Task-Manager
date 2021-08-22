use gtk::prelude::*;

use crate::core::view;

pub struct Application {
    parent: gtk::Application,
}

/// Main TaskManager's class
///
/// Initializes all the essential parts of the application
/// and manages its interactions
impl Application {

    /// Loading GResources into the application
    fn register_resources(&self) {
        let resource_bytes = include_bytes!("../../resources/resources.gresource");
        let resource_data = gtk::glib::Bytes::from(&resource_bytes[..]);
        let res = gtk::gio::Resource::from_data(&resource_data).unwrap();
        gtk::gio::resources_register(&res);
    }

    /// Creates new Application instance
    pub fn new() -> Self {
        let app: Application = Application {
            parent: gtk::Application::builder()
                .application_id("org.altereigo.task-manager")
                .build()
        };

        app.parent.connect_activate(|app| {
            let window = gtk::ApplicationWindow::builder()
                .application(app)
                .title("Æ Task Manager")
                .default_width(800)
                .default_height(600)
                .build();

            window.show();
        });
        app.register_resources();

        return app;
    }

    /// Runs the application's main loop
    pub fn run(&self) -> i32 {
        self.parent.run()
    }
}
