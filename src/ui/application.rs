use gtk::prelude::*;

pub struct Application {
    parent: gtk::Application,
}

/// Main TaskManager's class
///
/// Initializes all the essential parts of the application
/// and manages its interactions
impl Application {

    /// Creates new Application instance
    pub fn new() -> Application {
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

        return app;
    }

    /// Runs the application's main loop
    pub fn run(&self) -> i32 {
        self.parent.run()
    }
}
