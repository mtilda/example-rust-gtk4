mod custom_buildable;
mod squares_widget;

use custom_buildable::CustomBuildable;
use gtk::{glib, prelude::*};
use gtk4 as gtk;
use squares_widget::SquaresWidget;

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.squares")
        .build();
    app.connect_activate(load_hello_world_window);
    app.connect_activate(load_squares_widget_window);
    app.connect_activate(load_custom_buildable);
    app.run()
}

/** Load and present a window with the title "Hello, World!"
 * SOURCE: https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/index.html#hello-world-example-program
 */
fn load_hello_world_window(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .default_width(320)
        .default_height(200)
        .title("Hello, World!")
        .build();
    window.present();
}

/** Load and present a window with the Squares Widget example
 * SOURCE: https://github.com/gtk-rs/gtk4-rs/blob/master/examples/squares/main.rs
 */
fn load_squares_widget_window(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title(Some("Squares"));

    let widget = SquaresWidget::default();
    window.set_child(Some(&widget));
    window.present();
}

/** Load and present a window with the Custom Buildable example
 * SOURCE: https://github.com/gtk-rs/gtk4-rs/blob/master/examples/custom_buildable/main.rs
 */
fn load_custom_buildable(app: &gtk::Application) {
    CustomBuildable::static_type();

    let ui_src = include_str!("window.xml");
    let builder = gtk::Builder::from_string(ui_src);

    let window = builder
        .object::<gtk::ApplicationWindow>("window")
        .expect("Couldn't get window");
    app.add_window(&window);
    window.present();
}
