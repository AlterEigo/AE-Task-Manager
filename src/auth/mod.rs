use gtk::prelude::*;
use crate::prelude::{
    View
};

pub struct AuthView;
impl View for AuthView {
    fn assemble() -> gtk::Widget {
        let builder = gtk::Builder::from_resource("/org/altereigo/ae-task-manager/AuthFrame.glade");
        let grid: gtk::Grid = builder.object("root").unwrap();
        grid.show();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
