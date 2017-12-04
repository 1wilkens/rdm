extern crate rdmcommon;
extern crate rdmgreeter;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

extern crate gdk;
extern crate gdk_pixbuf;
extern crate gtk;

mod constants;
mod ui;

use std::rc::Rc;
use std::sync::Mutex;

use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};

fn setup_logger() -> Logger {
    let decor = TermDecorator::new().build();
    let drain = FullFormat::new(decor).build().fuse();
    let drain = Async::new(drain).build().fuse();
    let log = Logger::root(drain, o!());
    debug!(log, "Initialized logging");
    log
}

fn main() {
    let log = setup_logger();

    //let mut greeter = rdmgreeter::RdmGreeter::new(log).expect("Failed to get greeter");
    println!("Press any key to to continue");
    let mut res = String::new();
    let mut c = ::std::io::stdin().read_line(&mut res);

    // Init gtk
    (::gtk::init()).expect("Failed to initialize gtk");

    // Setup the Ui
    /*let mut ui = ui::Ui::from_theme(constants::THEME_NAME_DEFAULT, greeter);
    ui.show();*/

    // Start gtk event loop
    ::gtk::main();
}