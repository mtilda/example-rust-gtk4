mod squares_widget;
use gtk::{glib, prelude::*};
use gtk4 as gtk;
use squares_widget::SquaresWidget;

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.squares")
        .build();
    app.connect_activate(load_hello_world_window);
    app.connect_activate(load_squares_widget_window);
    app.run()
}

/** Example from gtk4-rs docs: https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/index.html#hello-world-example-program
 */
fn load_hello_world_window(app: &gtk::Application) {
    //
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .default_width(320)
        .default_height(200)
        .title("Hello, World!")
        .build();
    window.present();
}

/** Example from gtk4-rs examples: https://github.com/gtk-rs/gtk4-rs/blob/master/examples/squares/main.rs
 */
fn load_squares_widget_window(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title(Some("Squares"));

    let widget = SquaresWidget::default();
    window.set_child(Some(&widget));
    window.present();
}
