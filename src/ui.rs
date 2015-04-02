use constants::*;

use rgtk::*;
use rgtk::gtk::signals::{KeyPressEvent};
use rgtk::gtk::widgets::{Builder, Entry, Image, Window};

use std::path::PathBuf;

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

        let b = Builder::new_from_file(theme_file.to_str().unwrap()).expect("Failed to load default theme!");

        let w: Window = b.get_object(THEME_COMPONENT_WINDOW).expect("Failed to get main window from theme!");
        let bg: Image = b.get_object(THEME_COMPONENT_BG).expect("Failed to get background image from theme!");
        let user: Entry = b.get_object(THEME_COMPONENT_USER).expect("Failed to get user entry from theme!");
        let password: Entry = b.get_object(THEME_COMPONENT_PW).expect("Failed to get password entry from theme!");

        bg.set_from_file(bg_file.to_str().unwrap());

        RdmUi {
            window:     w,
            background: bg,
            user:       user,
            password:   password
        }
    }

    pub fn setup_events(&mut self) {
        Connect::connect(&self.user, KeyPressEvent::new(&mut |key|{
            let keyval = unsafe { (*key).keyval };
            if keyval == KEYVAL_ENTER {
                self.password.grab_focus();
            }
            false
        }));

        Connect::connect(&self.password, KeyPressEvent::new(&mut |key|{
            let keyval = unsafe { (*key).keyval };
            if keyval == KEYVAL_ENTER {
                let user = self.user.get_text().unwrap_or(String::new());
                let password = self.password.get_text().unwrap_or(String::new());
                let success = ::pam_auth::login("rdm", user.as_ref(), password.as_ref());
                if success {
                    gtk::main_quit();
                }
            }
            false
        }));
    }

    pub fn prepare_window(&mut self) {
        use rgtk::gtk::{WidgetTrait, WindowTrait};

        // fit to screen dimensions
        let (width, heigth) = (gdk::screen_width(), gdk::screen_height());
        self.window.set_default_size(width, heigth);

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
