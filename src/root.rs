use gtk::prelude::*;

use crate::auth::AuthView;
use crate::prelude::View;

pub struct RootView;
impl RootView {
    pub fn new() -> Self {
        RootView {}
    }

    fn assemble_auth() -> gtk::Widget {
        AuthView::new().assemble()
    }
}

impl View for RootView {
    fn assemble(&self) -> gtk::Widget {
        let grid = gtk::Grid::builder().build();
        let auth = RootView::assemble_auth();
        grid.attach(&auth, 0, 0, 1, 1);
        grid.show();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
