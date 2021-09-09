use gtk::prelude::*;

use crate::app::services::UserService;
use crate::auth::AuthView;
use crate::prelude::View;
use std::rc::Rc;

pub struct RootView {
    user_srv: Option<Rc<dyn UserService>>,
}

impl RootView {
    pub fn new() -> Self {
        RootView { user_srv: None }
    }

    pub fn user_service(self, value: &Rc<dyn UserService>) -> Self {
        RootView {
            user_srv: Some(value.clone()),
            ..self
        }
    }

    fn assemble_auth(&self) -> gtk::Widget {
        let mut view = AuthView::new();
        if let Some(srv) = &self.user_srv {
            view = view.user_service(&srv);
        };
        view.assemble()
    }
}

impl View for RootView {
    fn assemble(&self) -> gtk::Widget {
        let grid = gtk::Grid::builder().build();
        let auth = self.assemble_auth();
        grid.attach(&auth, 0, 0, 1, 1);
        grid.show();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
