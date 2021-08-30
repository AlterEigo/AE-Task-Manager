//!
//! TaskManager's root
//!

use gtk::prelude::*;

struct Application {
    gtk_app: gtk::Application,
}

impl Application {
    fn new() -> Self {
        Application {
            gtk_app: gtk::Application::builder()
                .application_id("Æ TaskManager")
                .build()
        }
    }

    fn run(&self) -> i32 {
        self.gtk_app.run()
    }
}
