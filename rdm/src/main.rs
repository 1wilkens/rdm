#![feature(conservative_impl_trait)]
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

mod login1;

use constants::*;

use std::io::{Read, Write};
use std::process::{Child, Command, exit};

use clap::{App, Arg, ArgMatches};
use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};

use dbus::{BusType, Connection, ConnectionItem, Error, Message, MessageItem, NameFlag};
use login1::OrgFreedesktopLogin1Manager;

fn test_dbus(log: &Logger) {
    let conn = match Connection::get_private(BusType::System) {
        Ok(c) => c,
        Err(e) => panic!("Manager: Failed to get DBUS connection: {:?}", e),
    };

    let conn_path = conn.with_path("org.freedesktop.login1", "/org/freedesktop/login1", 1000);
    let seats = conn_path.list_seats().expect("Failed to list seats");
    debug!(log, "Got Seats"; "seats" => ?seats);
    for (name, path) in seats {
        debug!(log, "Found seat"; "name" => name, "path" => ?path);
    }
}

fn run(matches: ArgMatches) -> Result<(), String> {
    let log = setup_logger();

    let mut display_mgr = displaymanager::DisplayManager::new();
    let mut seat_mgr = seatmanager::SeatManager::new();
    seat_mgr.add_seat("seat0");

    test_dbus(&log);
    //let mut ipc_mgr = ipc::IpcManager::new().expect("Failed to initialize IpcManager");
    //ipc::serve(move || Ok(ipc::IpcService::new(log.clone())));
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
