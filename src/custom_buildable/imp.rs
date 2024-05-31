// SOURCE: https://github.com/gtk-rs/gtk4-rs/blob/master/examples/custom_buildable/custom_buildable/imp.rs
use gtk::{glib, prelude::*, subclass::prelude::*};

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(file = "template.xml")]
pub struct CustomBuildable {
    #[template_child]
    pub box_: TemplateChild<gtk::Box>,
    #[template_child]
    pub prefixes: TemplateChild<gtk::Box>,
    #[template_child]
    pub suffixes: TemplateChild<gtk::Box>,
}

#[glib::object_subclass]
impl ObjectSubclass for CustomBuildable {
    const NAME: &'static str = "CustomBuildable";
    type Type = super::CustomBuildable;
    type ParentType = gtk::Widget;
    type Interfaces = (gtk::Buildable,);

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();

        // The layout manager determines how child widgets are laid out.
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CustomBuildable {
    // Needed for direct subclasses of GtkWidget;
    // Here you need to unparent all direct children
    // of your template.
    fn dispose(&self) {
        /* FIXME:
        
        Source code from the original example panics with the following error:

            error[E0599]: no method named `dispose_template` found for reference `&custom_buildable::imp::CustomBuildable` in the current scope

        Removing the line below, the program runs, but when the window is closed, the following warning is printed:

            Gtk-WARNING **: 19:23:52.941: Finalizing CustomBuildable 0x55bcfa532b90, but it still has children left:
            - GtkBox 0x55bcfa532ff0

        */
        // self.dispose_template();
    }
}

impl WidgetImpl for CustomBuildable {}

impl BuildableImpl for CustomBuildable {
    fn add_child(&self, builder: &gtk::Builder, child: &glib::Object, type_: Option<&str>) {
        let buildable = self.obj();
        // We first check if the main child `box_` has already been bound.
        if !self.box_.is_bound() {
            self.parent_add_child(builder, child, type_);
        } else if Some("prefix") == type_ {
            // Check if the child was added using `<child type="prefix">`
            buildable.add_prefix(child.downcast_ref::<gtk::Widget>().unwrap());
        } else if type_.is_none() {
            // Normal children
            buildable.add_suffix(child.downcast_ref::<gtk::Widget>().unwrap());
        };
    }
}
