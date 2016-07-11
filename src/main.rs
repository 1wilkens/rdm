#![allow(unused_imports, dead_code)]

extern crate env_logger;
extern crate dbus;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gtk;
extern crate pam_auth;

#[macro_use]
mod log;

mod constants;
mod manager;
mod ui;

use constants::*;
use ui::*;

use std::io::Write;
use std::process::Command;

#[cfg(not(feature = "debug"))]
fn start_x_server() {
    let x = Command::new(DEFAULT_SH_EXECUTABLE)
        .arg("-c")
        .arg(DEFAULT_X_EXECUTABLE)
        .arg(DEFAULT_X_DISPLAY)
        .arg(DEFAULT_X_VT)
        .spawn()
        .unwrap_or_else(|e| panic!("Failed to start X: {}", e));
}

#[cfg(feature = "debug")]
fn start_x_server() {

}

fn main() {
    env_logger::init().unwrap();
    start_x_server();

    //let mgr = manager::Manager::new();
    //mgr.start();

    // initialize gtk
    (::gtk::init()).expect("Failed to initialize gtk");

    // get ui components
    let mut ui = ui::Ui::from_theme("/home/florian/src/rust/rdm/theme/rdm.theme");

    // setup event handlers
    ui.setup_events();

    // show window
    ui.show();

    // start gtk main event loop
    ::gtk::main();
}
