#![allow(unused_imports, dead_code)]

extern crate env_logger;

extern crate dbus;
extern crate libc;

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

fn main() {
    env_logger::init().unwrap();

    match ::std::fs::create_dir(DEFAULT_RUN_DIR) {
        Ok(_)   => {},
        Err(e)  => panic!("Failed to create runtime dir: {}", e)
    }

    let mut x = server::Xserver::new();
    x.start();

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

    log_info!("[main] Exited gtk::main loop, cleaning up");
    x.stop();
    log_info!("[main] stopped X");

    let res = ::std::fs::remove_dir_all(DEFAULT_RUN_DIR);
    log_info!("[main] Remove runtime dir:={:?}", res);
    return;
}
