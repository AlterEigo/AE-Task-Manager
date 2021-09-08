use gtk::prelude::*;

use crate::app::services::UserService;
use crate::auth::AuthView;
use crate::prelude::View;

#[derive(Default)]
pub struct RootView<'a> {
    user_srv: Option<&'a dyn UserService>
}

impl<'a> RootView<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn user_service(self, value: &'a dyn UserService) -> Self {
        RootView {
            user_srv: Some(value),
            ..self
        }
    }
}

impl<'a> View for RootView<'a> {
    fn assemble(&self) -> gtk::Widget {
        let grid = gtk::Grid::builder().build();
        let auth = AuthView::new()
            .user_service(self.user_srv.unwrap())
            .assemble();
        grid.attach(&auth, 0, 0, 1, 1);
        grid.show();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
