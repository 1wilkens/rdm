#![allow(unused_imports, dead_code)]

extern crate env_logger;

extern crate dbus;

extern crate gdk;
extern crate gdk_pixbuf;
extern crate gtk;

extern crate pam_auth;
extern crate users;

extern crate rand;
extern crate uuid;

#[macro_use]
mod log;

mod constants;
mod manager;
mod ui;
mod server;

use constants::*;
use ui::*;

use std::io::Write;
use std::process::{Child, Command};

#[cfg(not(feature = "debug"))]
fn start_x_server() -> Child {
    log_info!("Executing '{} {} {} {}'", DEFAULT_X_EXECUTABLE, DEFAULT_X_ARGS, DEFAULT_X_DISPLAY, DEFAULT_X_VT);
    let process = Command::new(DEFAULT_X_EXECUTABLE)
        .arg(DEFAULT_X_ARGS)
        .arg(DEFAULT_X_DISPLAY)
        .arg(DEFAULT_X_VT)
        .spawn()
        .unwrap_or_else(|e| panic!("Failed to start X: {}", e));
    log_info!("Started X.. Sleeping 1 second");
    ::std::thread::sleep(::std::time::Duration::from_millis(1000));
    log_info!("Slept 1 second");
    return process;
}

#[cfg(feature = "debug")]
fn start_x_server() {

}

fn main() {
    env_logger::init().unwrap();

    let x = server::Xserver::new();
    return;

    let mut x = start_x_server();

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

    println!("Exited gtk::main loop, stopping X server");
    x.kill().expect("Failed to stop X server");
}
