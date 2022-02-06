extern crate backtrace;

use std::panic;

mod build;
mod handler;

fn main() {
    // handle panics
    panic::set_hook(Box::new(handler::panic_handler));

    // TODO
}
