use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use gdk::Screen;
use gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use gtk::{Builder, Entry, Image, Window};

use slog::Logger;

use rdmcommon::util;
use rdmgreeter::RdmGreeter;

use crate::constants::*;

// Regarding Rc/RefCell see: https://stackoverflow.com/a/31967816
pub struct Ui {
    window: Window,
    background: Image,
    user: Entry,
    secret: Entry,
    greeter: RefCell<RdmGreeter>,
    log: Logger,
}

#[allow(clippy::expect_fun_call)]
impl Ui {
    pub fn from_theme<T: AsRef<Path>>(theme_name: T, greeter: RdmGreeter) -> Rc<Self> {
        let theme_path = get_theme_path(theme_name, false);

        let mut theme_file = theme_path.clone();
        theme_file.push(THEME_MAIN_FILE_NAME);
        theme_file.set_extension(THEME_MAIN_FILE_EXT);

        let mut bg_file = theme_path.clone();
        bg_file.push(THEME_BACKGROUND_NAME);
        bg_file.set_extension(THEME_BACKGROUND_EXT);

        let b = Builder::new_from_file(theme_file.to_str().unwrap());

        let w: Window = b
            .get_object(THEME_COMPONENT_WINDOW)
            .expect("[ui]: Failed to get main window from theme!");
        let bg: Image = b
            .get_object(THEME_COMPONENT_BG)
            .expect("[ui]: Failed to get background image from theme!");
        let user: Entry = b
            .get_object(THEME_COMPONENT_USER)
            .expect("[ui]: Failed to get user entry from theme!");
        let secret: Entry = b
            .get_object(THEME_COMPONENT_SECRET)
            .expect("[ui]: Failed to get secret entry from theme!");

        // fit to screen dimensions
        let (width, heigth) = (Screen::width(), Screen::height());
        w.set_default_size(width, heigth);

        let pb =
            Pixbuf::new_from_file_at_scale(bg_file.to_str().unwrap(), width, heigth, false).expect(
                &format!("[ui]: Failed to get background image pixbuf: {:?}", bg_file),
            );

        bg.set_from_pixbuf(Some(&pb));

        let instance = Rc::new(Ui {
            window: w,
            background: bg,
            user: user,
            secret: secret,
            greeter: RefCell::from(greeter),
            log: util::plain_logger(),
        });

        {
            let secret = instance.secret.clone();

            instance.user.connect_key_release_event(move |_, e| {
                let val = (*e).get_keyval();
                if val == KEYVAL_ENTER {
                    secret.grab_focus();
                }
                Inhibit(true)
            });
        }

        {
            let _i = instance.clone();
            let user = instance.user.clone();
            let secret = instance.secret.clone();

            instance.secret.connect_key_release_event(move |_, e| {
                let val = (*e).get_keyval();
                if val == KEYVAL_ENTER {
                    let _user = user.get_text().unwrap_or_default();
                    let _password = secret.get_text().unwrap_or_default();

                    // TODO: use librdmgreeter to talk to daemon to authenticate
                    /*i.greeter.borrow_mut().request_authentication(&user, &password);*/
                }
                Inhibit(true)
            });
        }

        instance
    }

    pub fn show(&self) {
        info!(self.log, "[ui]: Showing window");
        self.window.show_all();
    }
}

/*fn start_session(name: &str) {
use users::os::unix::UserExt;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
use std::os::unix::process::CommandExt;

info!("[ui]: Starting session");

// Setup variables
let user = ::users::get_user_by_name(name)
    .expect(&format!("[ui]: Could not get user by name: {}!", name));
let dir = user.home_dir();
let shell = user.shell()
    .to_str()
    .expect("[ui]: Shell was not valid unicode!");

let args = format!("exec {} --login .xinitrc", shell);

// Need these later in `before_exec` to setup supplimentary groups
let name_c = CString::new(name).unwrap();
let uid = user.uid();
let gid = user.primary_group_id();

// Open session log file for stderr
//
// Currently we mirror sddm's behaviour which discards session's stdout
// and redirects stderr to a log file in the home directory
let stderr = File::create(format!("{}/{}", dir.to_str().unwrap(), DEFAULT_SESSION_LOG_FILE))
    .map(|f| {
        info!("[ui]: Redirecting session's stderr to {:?}", f);
        unsafe { Stdio::from_raw_fd(f.as_raw_fd()) }
    })
    .unwrap_or_else(|_| {
        info!("[ui]: Failed to create session log file, falling back to inherit..");
        Stdio::inherit()
    });

// Start session loading .xinitrc
info!("[ui]: Starting session");
info!("[ui]: Session command '{} -c {}'", shell, args);
/*let mut child = */
Command::new(shell)
// Arguments
.arg("-c")
.arg(args)
// Process setup
.current_dir(dir)
// TODO: Figure out why these are set / how to properly handle them
.env_remove("INVOCATION_ID")
.env_remove("JOURNAL_STREAM")
// Cannot use this as it does not add supplimentary groups
//.uid(user.uid())
.gid(gid)
.before_exec(move || {
// This sets the supplimentary groups for the session
if unsafe { initgroups(name_c.as_ptr(), gid) } != 0 {
error!("[ui:session] initgroups returned non-zero!");
Err(Error::last_os_error())
}
else if unsafe { setuid(uid) } != 0 {
error!("[ui:session] setuid returned non-zero!");
Err(Error::last_os_error())
}
else {
Ok(())
}
})
// Output redirection
.stdout(Stdio::null())
.stderr(stderr)
.spawn()
.unwrap_or_else(|e| panic!("[ui]: Failed to start session: {}!", e));

info!("[ui]: Spawned session process & waiting for result");

//TODO: Waiting for the child causes an "invisible window" in i3.. Investigate further
//let result = child.wait()
//    .unwrap_or_else(|e| panic!("[ui]: Failed to join session thread: {:?}!", e));
//info!("[ui]: Session exited with return code: {}", result);
}*/

fn get_theme_path<T: AsRef<Path>>(theme_name: T, default: bool) -> PathBuf {
    let mut theme_path = PathBuf::new();
    theme_path.push(THEME_BASE_PATH);
    theme_path.push(theme_name);

    if theme_path.is_dir() {
        theme_path
    } else if !default {
        get_theme_path(THEME_NAME_DEFAULT, true)
    } else {
        panic!("[ui]: Could not load default configuration!")
    }
}
