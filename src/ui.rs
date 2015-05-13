use constants::*;

use gtk::{Entry, Image, Window, EntryTrait, WidgetTrait, WindowTrait};
use gtk::widgets::{Builder};
use gtk::signal::{Inhibit, WidgetSignals};

use std::path::PathBuf;

use pam_auth::Authenticator;

pub struct RdmUi {
    pub window:     Window,
    pub background: Image,
    pub user:       Entry,
    pub password:   Entry
}

impl RdmUi {
    pub fn from_theme(theme_name: &str) -> RdmUi {
        let theme_path = get_theme_path(theme_name, false);

        let mut theme_file = theme_path.clone();
        theme_file.push(THEME_MAIN_FILE_NAME);
        theme_file.set_extension(THEME_MAIN_FILE_EXT);

        let mut bg_file = theme_path.clone();
        bg_file.push(THEME_BACKGROUND_NAME);
        bg_file.set_extension(THEME_BACKGROUND_EXT);

        let b = Builder::new_from_file(theme_file.to_str().unwrap())
            .expect("Failed to load default theme!");

        let w: Window = b.get_object(THEME_COMPONENT_WINDOW)
            .expect("Failed to get main window from theme!");
        let bg: Image = b.get_object(THEME_COMPONENT_BG)
            .expect("Failed to get background image from theme!");
        let user: Entry = b.get_object(THEME_COMPONENT_USER)
            .expect("Failed to get user entry from theme!");
        let password: Entry = b.get_object(THEME_COMPONENT_PW)
            .expect("Failed to get password entry from theme!");

        // fit to screen dimensions
        let (width, heigth) = (::gdk::screen_width(), ::gdk::screen_height());
        w.set_default_size(width, heigth);

        bg.set_from_file(bg_file.to_str().unwrap());

        RdmUi {
            window:     w,
            background: bg,
            user:       user,
            password:   password
        }
    }

    pub fn setup_events(&self) {
        let p_entry = self.password.clone();

        self.user.connect_key_release_event(move |_, e| {
            let val = (*e).keyval;
            if val == KEYVAL_ENTER {
                p_entry.grab_focus();
            }
            Inhibit(true)
        });

        let u_entry = self.user.clone();
        let p_entry = self.password.clone();

        self.password.connect_key_release_event(move |_, e| {
            let val = (*e).keyval;
            if val == KEYVAL_ENTER {
                let user = u_entry.get_text().unwrap_or(String::new());
                let password = p_entry.get_text().unwrap_or(String::new());

                let mut auth = Authenticator::new(APPLICATION_NAME)
                    .expect("Failed to get PAM authenticator!");
                auth.set_credentials(&user, &password);

                if auth.authenticate().is_ok() && auth.open_session().is_ok() {

                    ::gtk::main_quit();
                }
                else {
                    p_entry.set_text("");
                }
            }
            Inhibit(true)
        });
    }

    pub fn show(&mut self) {
        // show window
        self.window.show_all();
    }
}

fn get_theme_path(theme_name: &str, default: bool) -> PathBuf {
    use std::fs::PathExt;

    let mut theme_path = PathBuf::new();
    theme_path.push(THEME_BASE_PATH);
    theme_path.push(theme_name);

    if theme_path.is_dir() {
        theme_path
    }
    else if !default {
        get_theme_path(THEME_NAME_DEFAULT, true)
    }
    else {
        panic!("Could not load default configuration")
    }
}
