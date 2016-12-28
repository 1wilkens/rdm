use constants::*;

use gtk::prelude::*;

use gdk::{screen_width, screen_height};
use gdk_pixbuf::Pixbuf;
use gtk::{Builder, Entry, Image, Window};

use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::thread;

use pam_auth::Authenticator;

pub struct Ui {
    pub window:     Window,
    pub background: Image,
    pub user:       Entry,
    pub secret:     Entry
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
            .expect("[ui] Failed to get main window from theme!");
        let bg: Image = b.get_object(THEME_COMPONENT_BG)
            .expect("[ui] Failed to get background image from theme!");
        let user: Entry = b.get_object(THEME_COMPONENT_USER)
            .expect("[ui] Failed to get user entry from theme!");
        let secret: Entry = b.get_object(THEME_COMPONENT_SECRET)
            .expect("[ui] Failed to get secret entry from theme!");

        // fit to screen dimensions
        let (width, heigth) = (screen_width(), screen_height());
        w.set_default_size(width, heigth);

        let pb = Pixbuf::new_from_file_at_scale(bg_file.to_str().unwrap(), width, heigth, false)
            .ok().expect(&format!("[ui] Failed to get background image pixbuf: {:?}", bg_file));

        bg.set_from_pixbuf(Some(&pb));

        Ui {
            window:     w,
            background: bg,
            user:       user,
            secret:     secret
        }
    }

    pub fn setup_events(&self) {
        log_info!("[ui] Setting up events");
        let p_entry = self.secret.clone();

        self.user.connect_key_release_event(move |_, e| {
            let val = (*e).get_keyval();
            if val == KEYVAL_ENTER {
                p_entry.grab_focus();
            }
            Inhibit(true)
        });

        let u_entry = self.user.clone();
        let p_entry = self.secret.clone();
        let window = self.window.clone();

        self.secret.connect_key_release_event(move |_, e| {

            let val = (*e).get_keyval();
            if val == KEYVAL_ENTER {
                let user = u_entry.get_text().unwrap_or(String::new());
                let password = p_entry.get_text().unwrap_or(String::new());

                let mut auth = Authenticator::new(APPLICATION_NAME)
                    .expect("[ui] Failed to get PAM authenticator!");
                auth.close_on_drop = false;  //TODO: change this in release
                auth.set_credentials(&user, &password);

                let code1 = auth.authenticate();
                let code2 = auth.open_session();
                if code1.is_ok() && code2.is_ok() {
                    log_info!("[ui] Authentication successful! Hiding window");

                    window.hide();
                    xinit(&user);
                }
                else {
                    log_info!("[ui] authenticate={:?}, open_session={:?}", code1, code2);
                    p_entry.set_text("");
                }
            }
            Inhibit(true)
        });
    }

    pub fn show(&mut self) {
        log_info!("[ui] Showing window");
        self.window.show_all();
    }
}

fn xinit(name: &String) {
    use ::users::os::unix::UserExt;
    use std::os::unix::process::CommandExt;

    log_info!("[ui] Running 'xinit' functionality in new thread");

    let name = name.clone();
    let child = thread::spawn(move || {
        log_info!("[ui:child] Executing .xinitrc");

        // Setup variables
        let user = ::users::get_user_by_name(&name)
            .expect(&format!("[ui:child] Could not get user by name: {}!", name));
        let dir = user.home_dir();
        let shell = user.shell().to_str()
            .expect("[ui:child] Shell was not valid unicode!");

        let cmd_args = format!("exec {} --login .xinitrc", shell);

        // Load ~/.xinitrc
        log_info!("[ui:child] Load .xinitrc");
        log_info!("[ui:child] Session command '{} -c {}'", shell, cmd_args);
        Command::new(shell)
            .arg("-c")
            .arg(&cmd_args)
            .current_dir(dir)
            .uid(user.uid())
            .gid(user.primary_group_id())
            .spawn()
            .unwrap_or_else(|e| panic!("[ui:child] Failed to start session: {}!", e));
    });

    log_info!("[ui] Spawned session & waiting for result");
    let result = child.join();
    log_info!("[ui] Session ended with result: {:?}", result);
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
        panic!("[ui] Could not load default configuration!")
    }
}
