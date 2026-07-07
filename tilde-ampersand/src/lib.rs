pub mod container_items;
pub mod app;
pub mod container;
pub mod builders;
pub mod window;



// "Never trust a man who doesn't curse."
//              - A post on some LinkedIn profile by the guy who owned the 
//                company I reached out to asking if I could have the
//                'ampersand' namespace on crates.io
//                
//                Spoiler alert: he said no. :C

#[cfg(test)]
mod tests {
    #[test]
    fn test_app_draws_window() {
        use gtk::prelude::*;
        use glib::ControlFlow;

        gtk::init().expect("Failed to initialize GTK");

        let app = super::builders::AppBuilder::new("com.example.myapp").build();

        // Quit after 1 second so the test doesn't hang
        let app_quit = app.application.clone();
        glib::timeout_add_seconds_local(1, move || {
            app_quit.quit();
            ControlFlow::Break
        });

        app.run(|gtk_app| {
            let window = super::builders::WindowBuilder::new_from_app(gtk_app, "My Window".to_string())
                .set_decorated(true, true)
                .set_behavior(super::window::WindowBehavior::Tiled)
                .build();
            
            window.set_title("My Window");
            window.set_default_size(400, 300);
            
            vec![window]
        });
    }


}