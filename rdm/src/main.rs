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

fn main() {
    env_logger::init().expect("Failed to initialize env_logger");

    let mut mgr_display = displaymanager::DisplayManager::new();
    let mut mgr_seat = seatmanager::SeatManager::new();

    ipc::test_ipc();

    return;
}
