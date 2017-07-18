#![allow(unused_imports, dead_code)]
#![allow(useless_format)]

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate dbus;
extern crate libc;

extern crate pam_auth;
extern crate users;

extern crate rand;
extern crate uuid;

extern crate rdmcommon;

extern crate futures;
extern crate tokio_core;
extern crate tokio_service;
extern crate tokio_uds;

mod common;
mod constants;
mod displaymanager;
mod ipc;
mod manager;
mod seat;
mod seatmanager;
mod server;
mod session;

use constants::*;

use std::io::{Read, Write};
use std::process::{Child, Command};

use tokio_core::reactor::Core;

fn main() {
    env_logger::init().expect("Failed to initialize logger");

    let core = Core::new().expect("Failed to initialize core");

    let mut display_mgr = displaymanager::DisplayManager::new();
    let mut seat_mgr = seatmanager::SeatManager::new();
    seat_mgr.add_seat("seat0");
    
    let ipc_mgr = ipc::IpcManager::new(&core.handle()).expect("Failed to initialize IpcManager");

    return;
}
