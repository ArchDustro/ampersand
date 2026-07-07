use gtk::prelude::*;
use glib::ControlFlow;
use tilde_ampersand::{builders, container, container_items::label, window};

fn main() {
    gtk::init().expect("Failed to initialize GTK");

    let app = builders::AppBuilder::new("com.example.ampersand-window").build();

    // Close after 10 seconds
    let app_quit = app.application.clone();
    glib::timeout_add_seconds_local(10, move || {
        app_quit.quit();
        ControlFlow::Break
    });

    app.run(|app| {
        let window = builders::WindowBuilder::new_from_app(app, "Ampersand Window".to_string())
            .set_decorated(true, false)
            .set_behavior(window::WindowBehavior::Floating)
            .build();

        window.set_title("Ampersand - Tiled Window");
        window.set_default_size(500, 400);
        let container = window.add_container(container::ContainerOrientation::Horizontal, 16);
        container.add_label("Test", label::WrapMode::Word, 100, 16);
        vec![window]
    });
}
