#![feature(convert, path_ext)]

extern crate gdk;
extern crate gtk;
extern crate pam_auth;

mod constants;
mod ui;

use constants::*;
use ui::*;

//use gtk::*;

use std::process::Command;

fn start_x_server() {
    let x = Command::new(DEFAULT_SH_EXECUTABLE)
        .arg("-c")
        .arg(DEFAULT_X_EXECUTABLE)
        .arg(DEFAULT_X_DISPLAY)
        .arg(DEFAULT_X_VT)
        .spawn()
        .unwrap_or_else(|e| panic!("Failed to start X: {}", e));
}

fn main() {
    let test = true;

    if !test {
        start_x_server();
    }

    // initialize gtk
    ::gtk::init();

    // get ui components
    let mut ui = ui::RdmUi::from_theme("/home/florian/src/rust/rdm/res/ui.glade");

    // setup event handlers
    ui.setup_events();

    // show window
    ui.show();

    // start gtk main event loop
    ::gtk::main();
}
