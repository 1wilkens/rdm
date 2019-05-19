#![feature(async_await)]

#[macro_use]
extern crate slog;

mod constants;
mod ui;

use std::{io::stdin, rc::Rc, sync::Mutex};

use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};

fn setup_logger() -> Logger {
    let decor = TermDecorator::new().build();
    let drain = FullFormat::new(decor).build().fuse();
    let drain = Async::new(drain).build().fuse();
    let log = Logger::root(drain, o!());
    debug!(&log, "[setup_logger] Initialized logging");
    log
}

fn main() {
    // XXX: Readd when tokio-async-await gets updated
    /*tokio::run_async(*/async {
        let log = setup_logger();

        let _greeter = rdmgreeter::RdmGreeter::new(log.clone())
            .await
            .expect("Failed to get greeter");
        debug!(&log, "[main] Got greeter! Press any key to to continue");
        let mut res = String::new();
        let _c = stdin().read_line(&mut res);
        return;

        // Init gtk
        (::gtk::init()).expect("Failed to initialize gtk");

        // Setup the Ui
        /*let mut ui = ui::Ui::from_theme(constants::THEME_NAME_DEFAULT, greeter);
        ui.show();*/

        // Start gtk event loop
        ::gtk::main();
    }; //);
}
