use constants::*;

use libc::{close, pipe};

use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

// TODO: This should really be simpler I think
const HEX_CHARS : [char; 15]
    = ['0','1','2','3','4','5','6','7','8','9','a','b','c','e','f'];

pub struct Xserver {
    auth_cookie: String,
    auth_file:   Option<String>,
    process:     Option<Child>,
    display:     Option<String>
}

impl Xserver {
    pub fn new() -> Xserver {
        let cookie = generate_cookie();
        let file = touch_auth_file();

        Xserver {
            auth_cookie: cookie,
            auth_file: Some(file),
            process: None,
            display: None
        }
    }

    pub fn start(&mut self) {
        log_info!("[Xserver]: start()");

        if TESTING {
            // In TEST mode we don't start X
            // TODO: Start Xephyr here?
            return;
        }
        if self.process.is_some() {
            // X is already running -> silently return
            return;
        }

        // Set the previously generated auth cookie
        self.set_cookie();

        // Launch X and wait a second for it to start
        self.start_x_process();
    }

    pub fn stop(&mut self) {
        log_info!("[Xserver]: stop()");

        // TODO: There is probably more to do here
        // Kill X process
        if let Some(ref mut p) = self.process {
            log_info!("Killing X with PID={}", p.id());
            let res = p.kill();
            log_info!("Killed X. Result={:?}", res);
            if res.is_err() {
                log_info!("Failed to kill X: {}", res.err().unwrap());
            }
            p.wait().expect("[Xserver] Failed to wait for stopped X server!");
        }
        self.process = None;
        // Delete generated auth file
        match self.auth_file {
            None    => {},
            Some(ref f) => match fs::remove_file(&f) {
                Ok(_)   => {},
                Err(e)  => log_info!("Failed to delete x auth file: {}", e)
            }
        }
        self.auth_file = None;
    }

    fn set_cookie(&self) {
        log_info!("[Xserver]: Setting auth cookie");

        let auth_file = match self.auth_file {
            Some(ref f) => f,
            None        => panic!("[Xserver] Cannot set X auth cookie without an auth file!")
        };
        let mut auth = Command::new(XAUTH_EXECUTABLE)
            .arg("-f")
            .arg(auth_file)
            .arg("-q")
            .stdin(Stdio::piped())
            .spawn()
            .expect("[Xserver] Failed to spawn xauth process!");

        if let Some(ref mut pipe) = auth.stdin {
            pipe.write_all(b"remove :0\n").unwrap();
            pipe.write_all(format!("add :0 . {}\n", self.auth_cookie).as_bytes()).unwrap();
            pipe.write_all(b"exit\n").unwrap();
            // TODO: Maybe we don't need this
            pipe.flush().expect("[Xserver] Failed to flush xauth pipe!");
        }

        // Wait on xauth to prevent zombie processes
        auth.wait().unwrap();

        log_info!("[Xserver]: Auth cookie set");
    }

    fn start_x_process(&mut self) {
        use std::os::unix::io::FromRawFd;

        // Create pipes to read DISPLAY from X
        let mut fds = [0i32, 0i32];
        if unsafe { pipe(fds.as_mut_ptr()) != 0 } {
            panic!("[Xserver] Failed to open pipes to start X!");
        }

        let auth_file = match self.auth_file {
            Some(ref f) => f,
            None        => panic!("[Xserver] Cannot start X without an auth file!")
        };
        // Start X and set the field
        let child = Command::new(X_EXECUTABLE)
            .args(&X_DEFAULT_ARGS)
            .arg(X_ARG_DISPLAY_FD)
            .arg(&fds[1].to_string())
            .arg(X_DEFAULT_VT)
            //.arg(X_ARG_AUTH)
            //.arg(auth_file)
            .spawn()
            .unwrap_or_else(|e| panic!("[Xserver] Failed to start X: {}!", e));
        log_info!("self.process.id={}", child.id());
        self.process = Some(child);

        // Wait 2 seconds for X to start
        log_info!("[Xserver]: Started X.. Sleeping 2 seconds");
        ::std::thread::sleep(::std::time::Duration::from_millis(2000));
        log_info!("[Xserver]: Sleep finished");

        // Close writing end of the pipe
        unsafe { close(fds[1]) };

        // Read chosen DISPLAY from X and set the environment variable
        let mut pipe = unsafe { fs::File::from_raw_fd(fds[0]) };
        let mut display = String::new();
        pipe.read_to_string(&mut display)
            .expect("[Xserver] Failed to read DISPLAY from X!");
        // TODO: This allocates twice but we mostly deal with "0" so it shouldn't be a problem
        let display = format!(":{}", display.trim_right());
        env::set_var("DISPLAY", &display);
        log_info!("Got DISPLAY from X and set the env var: {}", &display);
        self.display = Some(display);

        // Close reading pipe // TODO: Is this necessary? Investigate
        //drop(pipe);
        unsafe { close(fds[0]) };
    }
}

impl Drop for Xserver {
    fn drop(&mut self) {
        // Currently we just stop the server
        self.stop();
    }
}

// Generate an authorization cookie
fn generate_cookie() -> String {
    log_info!("[Xserver]: Generating auth cookie");
    use rand::Rng;

    // TODO: replace this with another uuid?
    let mut cookie = String::with_capacity(32);
    let mut rng = ::rand::StdRng::new()
        .expect("[Xserver] Failed to get rng for cookie generation!");

    while cookie.len() < 32 {
        cookie.push(*rng.choose(&HEX_CHARS).unwrap());
    }

    log_info!("[Xserver]: Generated cookie: {}", &cookie);
    cookie
}

// Generate an auth file based on the run dir and a new uuid and touch it
fn touch_auth_file() -> String {
    log_info!("[Xserver]: Generating auth path");

    let uuid = ::uuid::Uuid::new_v4();
    let mut path = PathBuf::from(DEFAULT_RUN_DIR);
    path.push(uuid.hyphenated().to_string());

    match fs::OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(_)   => {},
        Err(e)  => panic!("[Xserver] Failed to touch X auth file: {}!", e)
    }

    log_info!("[Xserver]: Generated auth path: {:?}", &path);
    path.to_string_lossy().into_owned()
}
