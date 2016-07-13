use constants::*;

use std::path::PathBuf;
use std::fs::create_dir;

use uuid::Uuid;
use rand::StdRng;

// TODO: This should really be simpler I think
const HEX_CHARS : [char; 15]
    = ['0','1','2','3','4','5','6','7','8','9','a','b','c','e','f'];

pub struct Xserver {
    cookie: String,
    auth_path: String
}

impl Xserver {
    pub fn new() -> Xserver {
        let cookie = Xserver::generate_cookie();
        let path = Xserver::generate_auth_path();

        Xserver {
            cookie: cookie,
            auth_path: path
        }
    }

    // Generate an authorization cookie
    fn generate_cookie() -> String {
        use rand::Rng;

        let mut cookie = String::with_capacity(32);
        let mut rng = StdRng::new()
            .expect("Failed to get rng for cookie generation");

        while cookie.len() < 32 {
            cookie.push(*rng.choose(&HEX_CHARS).unwrap());
        }
        println!("generated cookie: {}", &cookie);

        cookie
    }

    // Generate an auth path based on the run dir and a new uuid
    fn generate_auth_path() -> String {
        let uuid = Uuid::new_v4();
        let mut path = PathBuf::from(DEFAULT_RUN_DIR);
        path.push(uuid.hyphenated().to_string());
        println!("generated auth path: {:?}", &path);

        path.to_string_lossy().into_owned()
    }
}
