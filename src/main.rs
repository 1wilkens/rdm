#![feature(path_ext)]

extern crate rgtk;

mod constants;
mod pam;

use constants::*;

use std::path::PathBuf;

use rgtk::*;
use rgtk::gtk::widgets::{Builder, Entry, Window};
use rgtk::gtk::signals::{KeyPressEvent};

struct RdmUi {
    pub window:     Window,
    pub user:       Entry,
    pub password:   Entry
}

impl RdmUi {
    pub fn from_theme(theme_name: &str) -> RdmUi {
        let theme_path = RdmUi::get_theme_path(theme_name);

        let mut theme_file = theme_path.clone();
        theme_file.push(THEME_MAIN_FILE_NAME);
        theme_file.set_extension(THEME_MAIN_FILE_EXT);

        let mut bg_file = theme_path.clone();
        bg_file.push(THEME_BACKGROUND_NAME);
        bg_file.set_extension(THEME_BACKGROUND_EXT);

        let b = Builder::new_from_file(theme_file.to_str().unwrap()).expect("Failed to load default theme!");

        let w: Window = b.get_object("window").expect("Failed to get main window from theme!");
        let user: Entry = b.get_object("user").expect("Failed to get user entry from theme!");
        let password: Entry = b.get_object("password").expect("Failed to get password entry from theme!");

        RdmUi {
            window:     w,
            user:       user,
            password:   password
        }
    }

    fn get_theme_path(theme_name: &str) -> PathBuf {
        use std::fs::PathExt;

        let mut theme_path = PathBuf::new(THEME_BASE_PATH);
        theme_path.push(theme_name);

        if theme_path.is_dir() {
            theme_path
        }
        else {
            RdmUi::get_theme_path(THEME_NAME_DEFAULT)
        }
    }
}

fn main() {

    // initialize gtk
    gtk::init();

    // get ui components
    let mut ui = RdmUi::from_theme("/home/florian/src/rust/rdm/res/ui.glade");

    // setup event handler
    Connect::connect(&ui.window, KeyPressEvent::new(&mut |key|{
        let keyval = unsafe { (*key).keyval };

        if keyval == KEYVAL_ENTER {
            if ui.user.is_focus() {
                println!("Enter key pressed in user entry");
                ui.password.grab_focus();
            }
            else if ui.password.is_focus() {
                println!("Enter key pressed in password entry");
                //TODO: perform login
            }
        }

        false
    }));

    //w.fullscreen();
    ui.window.show_all();

    // start gtk main event loop
    gtk::main();
}
