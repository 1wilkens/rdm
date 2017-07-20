extern crate rdmgreeter;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate gdk;
extern crate gdk_pixbuf;
extern crate gtk;

mod constants;
mod ui;

fn main() {
    // Init env_logger
    env_logger::init().expect("Failed to initialize env_logger");
    
    let mut greeter = rdmgreeter::RdmGreeter::new().expect("Failed to get greeter");
    println!("Got greeter.. press any key to to continue");
    let mut res = String::new();
    let mut c = ::std::io::stdin().read_line(&mut res);
    greeter.request_authentication();
    println!("Requested authentication.. press any key to to exit");
    c = ::std::io::stdin().read_line(&mut res);
    println!("{:?}", greeter);
    return;

    // Init gtk
    (::gtk::init()).expect("Failed to initialize gtk");

    // Setup the Ui
    let mut ui = ui::Ui::from_theme(constants::THEME_NAME_DEFAULT);
    ui.setup_events();
    ui.show();

    // Start gtk event loop
    ::gtk::main();
}