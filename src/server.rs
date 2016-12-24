use constants::*;

use libc::{close, pipe};

use std::fs::{copy, create_dir, remove_file, File, OpenOptions};
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
        self.start_x_process();

        // Set the previously generated auth cookie
        self.set_cookie();
    }

    pub fn stop(&mut self) {
        log_debug!("[Xserver]: stop()");
        // TODO: There is probably more to do here
        // Kill X process
        if let Some(ref mut p) = self.process {
            log_debug!("Killing X with PID={}", p.id());
            let res = p.kill();
            log_debug!("Killed X. Result={:?}", res);
            if res.is_err() {
                log_err!("Failed to kill X: {}", res.err().unwrap());
            }
            p.wait().expect("Failed to wait for stopped X server");
        }
        self.process = None;
        // Delete generated auth file
        match self.auth_file {
            None    => {},
            Some(ref f) => match remove_file(&f) {
                Ok(_)   => {},
                Err(e)  => log_err!("Failed to delete x auth file: {}", e)
            }
        }
        self.auth_file = None;
    }

    fn set_cookie(&self) {
        log_debug!("[Xserver]: Setting auth cookie");

        let auth_file = match self.auth_file {
            Some(ref f) => f,
            None        => panic!("Cannot start X without an auth file")
        };
        let mut auth = Command::new(XAUTH_EXECUTABLE)
            .arg("-f")
            .arg(auth_file)
            .arg("-q")
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to spawn xauth process");

        if let Some(ref mut pipe) = auth.stdin {
            pipe.write_all(b"remove :0\n").unwrap();
            pipe.write_all(format!("add :0 . {}\n", self.auth_cookie).as_bytes()).unwrap();
            pipe.write_all(b"exit\n").unwrap();
            // TODO: Maybe we don't need this
            pipe.flush().expect("Failed to flush xauth pipe");
        }

        // Wait on xauth to prevent zombie processes
        auth.wait().unwrap();

        log_debug!("[Xserver]: Auth cookie set");
    }

    // Start X process
    fn start_x_process(&mut self) {
        use std::os::unix::io::FromRawFd;

        // Create pipes to read DISPLAY from X
        let mut fds = [0i32, 0i32];
        if unsafe { pipe(fds.as_mut_ptr()) != 0 } {
            panic!("Failed to open pipes to start X");
        }

        // Start X and set the field
        let child = Command::new(X_EXECUTABLE)
            .args(&X_DEFAULT_ARGS)
            .arg(X_ARG_DISPLAY_FD)
            .arg(&fds[1].to_string())
            .arg(X_DEFAULT_VT)
            //.arg(X_ARG_AUTH)
            //.arg(auth_file)
            .spawn()
            .unwrap_or_else(|e| panic!("Failed to start X: {}", e));
        log_debug!("self.process.id={}", child.id());
        self.process = Some(child);

        // Wait 1 second for X to start
        log_debug!("[Xserver]: Started X.. Sleeping 1 second");
        ::std::thread::sleep(::std::time::Duration::from_millis(2000));
        log_debug!("[Xserver]: Sleep finished");

        // Close writing end of the pipe
        unsafe { close(fds[1]) };

        // Read chosen DISPLAY from X
        let mut pipe = unsafe { File::from_raw_fd(fds[0]) };
        let mut display = String::new();
        pipe.read_to_string(&mut display)
            .expect("Failed to read DISPLAY from X");
        self.display = Some(format!(":{}", display.trim_right()));

        // Close reading pipe // TODO: Is this necessary? Investigate
        //drop(pipe);
        //unsafe { close(fds[0]) };
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
