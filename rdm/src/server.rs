use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

use libc::{close, pipe};
use rand::{seq::SliceRandom, thread_rng};
use uuid::{
    adapter::{Hyphenated, Simple},
    Uuid,
};

use crate::constants::*;

// TODO: This should really be simpler I think
const HEX_CHARS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub struct Xserver {
    auth_cookie: String,
    auth_file: Option<String>,
    process: Option<Child>,
    display: Option<String>,
}

impl Xserver {
    pub fn new() -> Xserver {
        let cookie = generate_cookie();
        let file = touch_auth_file();

        Xserver {
            auth_cookie: cookie,
            auth_file: Some(file),
            process: None,
            display: None,
        }
    }

    pub fn start(&mut self) {
        //info!("[X]: start()");

        if TESTING {
            // In TESTING mode we don't start X
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
        //info!("[X]: stop()");

        // TODO: There is probably more to do here
        // Kill X process
        if let Some(ref mut p) = self.process {
            //info!("[X]: Killing X with PID={}", p.id());
            match p.kill() {
                Ok(_res) => {}  //info!("[X]: Killed X with Result={:?}", res),
                Err(_err) => {} //error!("[X]: Failed to kill X: {}", err),
            };

            p.wait().expect("[X]: Failed to wait for stopped X server!");
        }
        self.process = None;
        // Delete generated auth file
        match self.auth_file {
            None => {}
            Some(ref f) => {
                match fs::remove_file(&f) {
                    Ok(_) => {}
                    Err(_e) => {} //info!("Failed to delete x auth file: {}", e),
                }
            }
        }
        self.auth_file = None;
    }

    fn set_cookie(&self) {
        //info!("[X]: Setting auth cookie");

        let auth_file = match self.auth_file {
            Some(ref f) => f,
            None => panic!("[X]: Cannot set X auth cookie without an auth file!"),
        };
        let mut auth = Command::new(XAUTH_EXECUTABLE)
            .arg("-f")
            .arg(auth_file)
            .arg("-q")
            .stdin(Stdio::piped())
            .spawn()
            .expect("[X]: Failed to spawn xauth process!");

        if let Some(ref mut pipe) = auth.stdin {
            pipe.write_all(b"remove :0\n").unwrap();
            pipe.write_all(format!("add :0 . {}\n", self.auth_cookie).as_bytes())
                .unwrap();
            pipe.write_all(b"exit\n").unwrap();
            pipe.flush().expect("[X]: Failed to flush xauth pipe!");
        }

        // Wait on xauth to prevent zombie processes
        auth.wait().expect("[X]: Failed to wait on xauth process!");

        //info!("[X]: Auth cookie set");
    }

    fn start_x_process(&mut self) {
        use std::os::unix::io::FromRawFd;

        // Create pipes to read DISPLAY from X
        let mut fds = [0i32, 0i32];
        if unsafe { pipe(fds.as_mut_ptr()) != 0 } {
            panic!("[X]: Failed to open pipes to start X!");
        }

        let _auth_file = match self.auth_file {
            Some(ref f) => f,
            None => panic!("[X]: Cannot start X without an auth file!"),
        };
        // Start X and set the field
        let child = Command::new(X_EXECUTABLE)
            // Arguments
            .args(&X_DEFAULT_ARGS)
            .arg(X_ARG_DISPLAY_FD)
            .arg(&fds[1].to_string())
            .arg(X_DEFAULT_VT)
            //.arg(X_ARG_AUTH)
            //.arg(auth_file)
            // Output redirection
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap_or_else(|e| panic!("[X]: Failed to start X: {}!", e));
        self.process = Some(child);

        // Wait 1 second for X to start
        //info!("[X]: Started X.. Sleeping 1 second");
        thread::sleep(Duration::from_millis(1000));
        //info!("[X]: Sleep finished");

        // Close writing end of the pipe
        unsafe { close(fds[1]) };

        // Read chosen DISPLAY from X and set the environment variable
        let mut pipe = unsafe { fs::File::from_raw_fd(fds[0]) };
        let mut display = String::new();
        pipe.read_to_string(&mut display)
            .expect("[X]: Failed to read DISPLAY from X!");
        // TODO: This allocates twice but we mostly deal with "0" so it shouldn't be a problem
        let display = format!(":{}", display.trim_end());
        env::set_var("DISPLAY", &display);
        //debug!("[ui]: Got DISPLAY from X and set the env var: {}", &display);
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
    //info!("[X]: Generating auth cookie");

    // We use an uuid in 'simple' format aka 32 random chars
    Simple::from_uuid(Uuid::new_v4()).to_string()
}

// Generate an auth file based on the run dir and a new uuid and touch it
fn touch_auth_file() -> String {
    //info!("[X]: Generating auth path");

    let uuid = Uuid::new_v4();
    let mut path = PathBuf::from(DEFAULT_RUN_DIR);
    path.push(Hyphenated::from_uuid(uuid).to_string());

    match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
    {
        Ok(_) => {}
        Err(e) => panic!("[X]: Failed to touch X auth file: {}!", e),
    }

    //debug!("[X]: Generated auth path: {:?}", &path);
    path.to_string_lossy().into_owned()
}
