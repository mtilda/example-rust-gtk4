// SOURCE: https://github.com/gtk-rs/gtk4-rs/blob/master/examples/custom_buildable/custom_buildable/mod.rs
mod imp;
use gtk::{glib, prelude::*, subclass::prelude::*};
use gtk4 as gtk;

glib::wrapper! {
    pub struct CustomBuildable(ObjectSubclass<imp::CustomBuildable>)
        @extends gtk::Widget,
        @implements gtk::Buildable;
}

impl CustomBuildable {
    pub fn add_suffix<T: IsA<gtk::Widget>>(&self, widget: &T) {
        let imp = self.imp();
        imp.suffixes.append(widget);
        imp.suffixes.set_visible(true);
    }

    pub fn add_prefix<T: IsA<gtk::Widget>>(&self, widget: &T) {
        let imp = self.imp();
        imp.prefixes.append(widget);
        imp.prefixes.set_visible(true);
    }
}
