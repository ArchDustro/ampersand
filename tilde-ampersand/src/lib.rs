mod window;
mod box_items;
mod app;
mod container;

// Too little code for its own file so I'll just slap it here.
// I'm a genius programmer, I know.
pub enum Error {
    Message(String),
    Code(u32)
}

// "Never trust a man who doesn't curse."