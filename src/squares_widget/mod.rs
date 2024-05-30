// SOURCE: https://github.com/gtk-rs/gtk4-rs/blob/master/examples/squares/squares_widget/mod.rs
mod imp;
use gtk4 as gtk;

use gtk::glib;

glib::wrapper! {
    pub struct SquaresWidget(ObjectSubclass<imp::SquaresWidget>)
        @extends gtk::Widget;
}

impl Default for SquaresWidget {
    fn default() -> Self {
        glib::Object::new()
    }
}
