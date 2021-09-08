use gtk::prelude::*;

use std::rc::Rc;
use crate::app::services::UserService;
use crate::auth::AuthView;
use crate::prelude::View;

pub struct RootView {
    user_srv: Option<Rc<dyn UserService>>,
}

impl RootView {
    pub fn new() -> Self {
        RootView {
            user_srv: None
        }
    }

    pub fn user_service(self, value: Rc<dyn UserService>) -> Self {
        RootView {
            user_srv: Some(value),
            ..self
        }
    }
}

impl View for RootView {
    fn assemble(&self) -> gtk::Widget {
        let grid = gtk::Grid::builder().build();
        let usrv: Rc<dyn UserService> = self.user_srv.as_ref().unwrap().clone();
        let auth = AuthView::new()
            .user_service(usrv)
            .assemble();
        grid.attach(&auth, 0, 0, 1, 1);
        grid.show();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
