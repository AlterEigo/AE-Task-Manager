//!
//! TaskManager's root
//!

use gtk::prelude::*;

struct Application {
    gtk_app: gtk::Application,
}

impl Application {
    fn new() -> Self {
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

    fn run(&self) -> i32 {
        self.gtk_app.run()
    }
}
