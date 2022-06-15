#![warn(rust_2018_idioms)]
#![allow(unused_imports, dead_code)]
#![allow(clippy::useless_format, clippy::redundant_field_names)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate slog;

mod bus;
mod constants;
mod ipc;
mod server;
mod session;

use crate::constants::*;

use std::io::{Read, Write};
use std::process::{exit, Child, Command};

use clap::{App, Arg, ArgMatches};
use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};

use tokio::signal::{
    ctrl_c,
    unix::{signal, SignalKind},
};
use tokio::time::{self, Duration};

//fn test_dbus(log: &Logger) {
//    let conn = match Connection::get_private(BusType::System) {
//        Ok(c) => c,
//        Err(e) => panic!("Manager: Failed to get DBUS connection: {:?}", e),
//    };
//
//    let conn_path = conn.with_path("org.freedesktop.login1", "/org/freedesktop/login1", 1000);
//    let seats = conn_path.list_seats().expect("Failed to list seats");
//    debug!(log, "Got Seats"; "seats" => ?seats);
//    for (name, path) in seats {
//        debug!(log, "Found seat"; "name" => name, "path" => ?path);
//    }
//}

fn setup_logger() -> Logger {
    let decor = TermDecorator::new().build();
    let drain = FullFormat::new(decor).build().fuse();
    let drain = Async::new(drain).build().fuse();
    Logger::root(drain, o!())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let matches = App::new("rdm")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Enable verbose output"),
        )
        .get_matches();

    let log = setup_logger();
    info!(&log, "[main] Initialized logging");

    let mut bus = bus::BusManager::new(log.clone()).expect("Failed to init dbus");
    info!(&log, "[main] Initialized DBus");

    // XXX: Introduce error type with from conversion
    //let conn =
    // Connection::get_private(BusType::System).map_err(|_| "Failed to get DBUS connection")?;

    /*let conn =
        Connection::get_private(BusType::System).map_err(|_| "Failed to get DBUS connection")?;
    let foo = dbus::Manager::from_conn(conn).map_err(|_| "Failed to get DBUS connection")?;*/

    // XXX: Lets start this simple
    // 1. init logging and read config
    // 2. init ipc socket (and start listening?)
    // 3. start x
    // 4. start greeter (binary from config?)
    // 5. auth
    // 6. start session (and kill x?)
    // 7. wait for session exit -> goto 3
    //

    let mut ipc = ipc::IpcManager::new(log.clone()).expect("Failed to init ipc");
    info!(&log, "[main] Initialized IPC");

    let mut signals =
        signal(SignalKind::hangup()).map_err(|_| "Failed to setup signal handling")?;
    info!(&log, "[main] Entering event loop");
    loop {
        tokio::select! {
            res = bus.run() => {
                debug!(&log, "[main] Bus result: {:?}", res);
                break;
            }
            res = ipc.run() => {
                debug!(&log, "[main] IPC result: {:?}", res);
                break;
            }
            _ = signals.recv() => {
                info!(&log, "[main] SIGHUP => reloading configuration");
            }
            _ = ctrl_c() => {
                info!(&log, "[main] SIGINT => exiting");
                break;
            }
        }
    }

    Ok(())
}
