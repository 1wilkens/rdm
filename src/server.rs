use constants::*;

use std::fs::{copy, create_dir, remove_file, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

// TODO: This should really be simpler I think
const HEX_CHARS : [char; 15]
    = ['0','1','2','3','4','5','6','7','8','9','a','b','c','e','f'];

pub struct Xserver {
    auth_cookie: String,
    auth_file:   String,
    process:     Option<Child>
}

impl Xserver {
    pub fn new() -> Xserver {
        let cookie = generate_cookie();
        let file = touch_auth_file();

        Xserver {
            auth_cookie: cookie,
            auth_file: file,
            process: None
        }
    }

    pub fn start(&mut self) {
        log_debug!("[Xserver]: start()");

        if TESTING {
            // In TEST mode we don't start X
            // TODO: Start Xephyr here?
            return;
        }
        if self.process.is_some() {
            // X is already running -> silently return
            return;
        }

        // Launch X and wait a second for it to start
        self.process = Some(start_x_process(&self.auth_file));
        log_debug!("[Xserver]: Started X.. Sleeping 1 second");
        ::std::thread::sleep(::std::time::Duration::from_millis(1000));
        log_debug!("[Xserver]: Sleep finished");

        // Set the previously generated auth cookie
        self.set_cookie();
    }

    pub fn stop(&mut self) {
        log_debug!("[Xserver]: stop()");
        // TODO: There is probably more to do here
        if let Some(ref mut p) = self.process {
            let res = p.kill();
            if res.is_err() {
                log_err!("Failed to kill X: {}", res.err().unwrap());
            }
        }
        self.process = None;
    }

    fn set_cookie(&self) {
        log_debug!("[Xserver]: Setting auth cookie");

        let mut auth = Command::new(XAUTH_EXECUTABLE)
            .arg("-f")
            .arg(&self.auth_file)
            .arg("-q")
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to spawn xauth process");

        if let Some(ref mut pipe) = auth.stdin {
            pipe.write_all(b"remove :0\n").unwrap();
            pipe.write_all(format!("add :0 . {}\n", self.auth_cookie).as_bytes()).unwrap();
            pipe.write_all(b"exit\n").unwrap();
            pipe.flush().expect("Failed to flush xauth pipe");
        }

        match auth.wait() {
            Ok(_)   => {},
            Err(e)  => log_err!("Failed to wait for xauth process: {}", e)
        }

        log_debug!("[Xserver]: Auth cookie set");
    }
}

impl Drop for Xserver {
    fn drop(&mut self) {
        // Currently we just stop the server
        self.stop();
        match remove_file(&self.auth_file) {
            Ok(_)   => {},
            Err(e)  => log_err!("Failed to delete x auth file: {}", e)
        }
    }
}

// Generate an authorization cookie
fn generate_cookie() -> String {
    log_debug!("[Xserver]: Generating auth cookie");
    use rand::Rng;

    // TODO: replace this with another uuid?
    let mut cookie = String::with_capacity(32);
    let mut rng = ::rand::StdRng::new()
        .expect("Failed to get rng for cookie generation");

    while cookie.len() < 32 {
        cookie.push(*rng.choose(&HEX_CHARS).unwrap());
    }

    log_debug!("[Xserver]: Generated cookie: {}", &cookie);
    cookie
}

// Generate an auth file based on the run dir and a new uuid and touch it
fn touch_auth_file() -> String {
    log_debug!("[Xserver]: Generating auth path");

    let uuid = ::uuid::Uuid::new_v4();
    let mut path = PathBuf::from(DEFAULT_RUN_DIR);
    path.push(uuid.hyphenated().to_string());

    match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(_)   => {},
        Err(e)  => panic!("Failed to touch x auth file: {}", e)
    }

    log_debug!("[Xserver]: Generated auth path: {:?}", &path);
    path.to_string_lossy().into_owned()
}

// Start X process
fn start_x_process(auth_file: &str) -> Child {
    Command::new(X_EXECUTABLE)
        .args(&X_DEFAULT_ARGS)
        .arg(X_DEFAULT_DISPLAY)
        .arg(X_DEFAULT_VT)
        //.arg(X_AUTH_ARG)
        //.arg(auth_file)
        .spawn()
        .unwrap_or_else(|e| panic!("Failed to start X: {}", e))
}
