# Ampersand Example
This is an example Rust program that uses the Ampersand API.

## Setup
Firstly, create your rust program. (I use cargo as I don't even know of any alternatives)
```console
┏━┫10:01┣━┫dus┣━┫garbage-collector┃
┃
┗━┫~/Projects┣━┫$: cargo new tilde-test
    Creating binary (application) `tilde-test` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```
Alternatively;
```console
┏━┫10:02┣━┫dus┣━┫garbage-collector┃
┃
┗━┫~/Projects┣━┫$: mkdir tilde-test

┏━┫10:03┣━┫dus┣━┫garbage-collector┃
┃
┗━┫~/Projects┣━┫$: cd tilde-test

┏━┫10:03┣━┫dus┣━┫garbage-collector┃
┃
┗━┫~/Projects/tilde-test┣━┫$: cargo init
    Creating binary (application) package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

Secondly, add your imports.
```console
┏━┫10:03┣━┫dus┣━┫garbage-collector┃
┃
┗━┫~/Projects/tilde-test┣━┫$: cargo add tilde-ampersand
    Updating crates.io index
      Adding tilde-ampersand v0.2.0 to dependencies
    Updating crates.io index
     Locking 89 packages to latest Rust 1.96.1 compatible versions
      Adding toml v0.8.2 (available: v0.8.23)
      Adding toml_datetime v0.6.3 (available: v0.6.11)
      Adding toml_edit v0.20.2 (available: v0.20.7)
# Note: the crate is 'tilde-ampersand', as unfortunately the 'ampersand' crate was taken.

┏━┫10:05┣━┫dus┣━┫garbage-collector┃
┃
┗━┫~/Projects/tilde-test┣━┫$: cargo add gtk glib
    Updating crates.io index
      Adding gtk v0.18.2 to dependencies
             Features:
#               You will see a list of features
      Adding glib v0.22.8 to dependencies
             Features:
#               You will see a list of features
```

Next, open your code editor in the directory of your program.
```console
# I use codium, so I would do
┏━┫10:06┣━┫dus┣━┫garbage-collector┃
┃
┗━┫~/Projects/tilde-test┣━┫$: codium .
# Note: the '.' tells codium to open the IDE in this directory.
# Alternatively, you can do:
┏━┫10:08┣━┫dus┣━┫garbage-collector┃
┃
┗━┫~/Projects/tilde-test┣━┫$: codium ./src/
# Which will open the IDE in the src/ folder.
# I prefer to open my IDE in the root of the project, as it gives me easy access to the target/ directory and Cargo.toml file.
```
You have set up your program. Now you can navigate to your src/main.rs file.

## Example Program
You can copy this straight into your src/main.rs file.
Expected outcome: A window with some text and a button will appear for 10 seconds and then disappear, exiting the program.
```rust
use gtk::prelude::*;
use glib::ControlFlow;
use tilde_ampersand::{builders, container, container_items::label::*, window};
fn main() {
    // You must intialise GTK.
    gtk::init().expect("Failed to initialize GTK");

    // Next, create your app with your application ID.
    let app = builders::AppBuilder::new("io.github.archdustro.tildetest").build();

    // Clone the app and set a rule to close it after 10 seconds
    let app_quit = app.application.clone();
    glib::timeout_add_seconds_local(10, move || {
        app_quit.quit();
        ControlFlow::Break
    });

    // Pass a closure to the app to define its activity
    app.run(|app| {
        // Create a new window. This is what actually creates a window in your environment.
        let window = builders::WindowBuilder::new_from_app(app, "Ampersand Window".to_string())
            .set_decorated(true, false)
            .build();

        // Name the title of the new window.
        window.set_title("Ampersand Test App");
        window.set_default_size(500, 400);

        // We add a container to the window so it can hold items such as a label (displays text) and a button.
        let container = window.add_container(
            // This defines the orientation of the container.
            container::ContainerOrientation::Horizontal, 
            // This tells the container how close its children can get to the sides of the window.
            16);

        // This will add a label with the phrase "Press the button!" on it
        let mut label = container.add_label(
            "Press the button!",
            WrapMode::Word,
            100,
            16,
        );

        label.set_alignment(Alignment::Centre);

        // This will add a button. You must clone the label to change it.
        let label_for_button = label.clone();

        let mut button = container.add_buttom(
            "Press me!",
            Box::new(move || {
                label_for_button.set_text("Button pressed!");
            }),
            16,
        );

        vec![window]
    });
}
```