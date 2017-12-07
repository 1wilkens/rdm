#![allow(unused_imports, dead_code)]
#![allow(useless_format)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

extern crate dbus;
extern crate libc;

extern crate pam_auth;
extern crate users;

extern crate rand;
extern crate uuid;

extern crate rdmcommon;

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_service;
//extern crate tokio_proto;
extern crate tokio_uds;
//extern crate tokio_uds_proto;

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
use std::process::{Child, Command, exit};

use clap::{App, Arg, ArgMatches};
use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};

fn run(matches: ArgMatches) -> Result<(), String> {
    let log = setup_logger();

    let mut display_mgr = displaymanager::DisplayManager::new();
    let mut seat_mgr = seatmanager::SeatManager::new();
    seat_mgr.add_seat("seat0");
    
    //let mut ipc_mgr = ipc::IpcManager::new().expect("Failed to initialize IpcManager");
    ipc::serve(|| Ok(ipc::IpcService));
    //ipc_mgr.start();

    Ok(())
}

fn setup_logger() -> Logger {
    let decor = TermDecorator::new().build();
    let drain = FullFormat::new(decor).build().fuse();
    let drain = Async::new(drain).build().fuse();
    let log = Logger::root(drain, o!());
    debug!(log, "Initialized logging");
    log
}

fn main() {
    let matches = App::new("rdm")
        .version("0.1")
        .about("Rust Display Manager (working title)")
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Enable verbose output"))
        .get_matches();

    if let Err(e) = run(matches) {
        //error!("Application error: {}", e);
        std::process::exit(1);
    }
}
