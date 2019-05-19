#![feature(async_await)]
#![allow(unused_imports, dead_code)]
#![allow(clippy::useless_format, clippy::redundant_field_names)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate slog;

#[macro_use]
extern crate tokio;

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

use crate::constants::*;

use std::io::{Read, Write};
use std::process::{exit, Child, Command};

use clap::{App, Arg, ArgMatches};
use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};

use crate::login1::OrgFreedesktopLogin1Manager;
use dbus::{BusType, Connection, ConnectionItem, Error, Message, MessageItem, NameFlag};

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

fn run(_matches: ArgMatches) -> Result<(), String> {
    let log = setup_logger();

    let _display_mgr = displaymanager::DisplayManager::new();
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
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Enable verbose output"),
        )
        .get_matches();

    if let Err(_e) = run(matches) {
        //error!("Application error: {}", e);
        std::process::exit(1);
    }
}
