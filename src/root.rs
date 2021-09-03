use gtk::prelude::*;

use crate::auth::{AuthView};
use crate::prelude::{
    View
};

pub struct RootView;
impl View for RootView {
    fn assemble() -> gtk::Widget {
        let grid = gtk::Grid::builder().build();
        let auth = AuthView::assemble();
        grid.attach(&auth, 0, 0, 1, 1);
        grid.show();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
