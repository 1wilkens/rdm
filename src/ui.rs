use constants::*;

use gdk::{screen_width, screen_height};
use gdk_pixbuf::Pixbuf;
use gtk::{Builder, Entry, Image, Inhibit, Window, EntryExt, WidgetExt, WindowExt};

use std::path::PathBuf;

use pam_auth::Authenticator;

pub struct Ui {
    pub window:     Window,
    pub background: Image,
    pub user:       Entry,
    pub password:   Entry
}

impl Ui {
    pub fn from_theme(theme_name: &str) -> Ui {
        let theme_path = get_theme_path(theme_name, false);

        let mut theme_file = theme_path.clone();
        theme_file.push(THEME_MAIN_FILE_NAME);
        theme_file.set_extension(THEME_MAIN_FILE_EXT);

        let mut bg_file = theme_path.clone();
        bg_file.push(THEME_BACKGROUND_NAME);
        bg_file.set_extension(THEME_BACKGROUND_EXT);


        let b = Builder::new_from_file(theme_file.to_str().unwrap());

        let w: Window = b.get_object(THEME_COMPONENT_WINDOW)
            .expect("Failed to get main window from theme!");
        let bg: Image = b.get_object(THEME_COMPONENT_BG)
            .expect("Failed to get background image from theme!");
        let user: Entry = b.get_object(THEME_COMPONENT_USER)
            .expect("Failed to get user entry from theme!");
        let password: Entry = b.get_object(THEME_COMPONENT_PW)
            .expect("Failed to get password entry from theme!");

        // fit to screen dimensions
        let (width, heigth) = (screen_width(), screen_height());
        w.set_default_size(width, heigth);

        let pb = Pixbuf::new_from_file_at_scale(bg_file.to_str().unwrap(), width, heigth, false)
            .ok().expect(&format!("Failed to get background image pixbuf: {:?}", bg_file));

        bg.set_from_pixbuf(Some(&pb));

        Ui {
            window:     w,
            background: bg,
            user:       user,
            password:   password
        }
    }

    pub fn setup_events(&self) {
        let p_entry = self.password.clone();

        self.user.connect_key_release_event(move |_, e| {
            let val = (*e).get_keyval();
            if val == KEYVAL_ENTER {
                p_entry.grab_focus();
            }
            Inhibit(true)
        });

        let u_entry = self.user.clone();
        let p_entry = self.password.clone();

        self.password.connect_key_release_event(move |_, e| {
            let val = (*e).get_keyval();
            if val == KEYVAL_ENTER {
                let user = u_entry.get_text().unwrap_or(String::new());
                let password = p_entry.get_text().unwrap_or(String::new());

                let mut auth = Authenticator::new(APPLICATION_NAME)
                    .expect("Failed to get PAM authenticator!");
                auth.close_on_drop = true;  //TODO: change this in release
                auth.set_credentials(&user, &password);

                let code1 = auth.authenticate();
                let code2 = auth.open_session();
                if code1.is_ok() && code2.is_ok() {
                    println!("Auth okay sleeping for 10 seconds");
                    ::std::thread::sleep(::std::time::Duration::new(10, 0));
                    println!("Sleep finished. Exiting now");
                    ::gtk::main_quit();
                }
                else {
                    println!("authenticate={:?}, open_session={:?}", code1, code2);
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
