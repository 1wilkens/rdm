/* BASE CONSTANTS */
pub const APPLICATION_NAME: &str = "rdm";

/* DBUS CONSTANTS */
pub const DBUS_SERVICE_NAME: &str = "com.github.1wilkens.RDM";
pub const DBUS_ROOT_PATH: &str = "com/github/1wilkens/RDM";

/* -- CONFIG SETTINGS -- */
pub const DEFAULT_CONFIG_FILE: &str = "/etc/rdm.conf";
pub const DEFAULT_RUN_DIR: &str = "/var/run/rdm";
pub const DEFAULT_SESSION_LOG_FILE: &str = "session.log";

// Change this to prevent automatic X server start e.g.
pub const TESTING: bool = false;

/* -- EXECUTABLES / DIRECTORIES -- */
pub const SH_EXECUTABLE: &str = "/bin/sh";
pub const X_EXECUTABLE: &str = "/usr/bin/X";
pub const XAUTH_EXECUTABLE: &str = "/usr/bin/xauth";

pub const XSESSIONS_DIRECTORY: &str = "/usr/share/xsessions";

/* -- X SERVER SETTINGS --*/
pub const X_DEFAULT_ARGS: [&str; 5] = ["-nolisten", "tcp", "-logverbose", "75", "-noreset"];
pub const X_DEFAULT_DISPLAY: &str = ":0";
pub const X_DEFAULT_VT: &str = "vt01";
pub const X_ARG_AUTH: &str = "-auth";
pub const X_ARG_DISPLAY_FD: &str = "-displayfd";
pub const X_AUTHORITY_FILE: &str = ".Xauthority";

/* -- THEME SETTINGS -- */
pub const THEME_BASE_PATH: &str = "/usr/share/rdm/themes/";

pub const THEME_MAIN_FILE_NAME: &str = "rdm";
pub const THEME_MAIN_FILE_EXT: &str = "theme";

pub const THEME_BACKGROUND_NAME: &str = "background";
pub const THEME_BACKGROUND_EXT: &str = "png";

pub const THEME_COMPONENT_WINDOW: &str = "window";
pub const THEME_COMPONENT_BG: &str = "background";
pub const THEME_COMPONENT_USER: &str = "user";
pub const THEME_COMPONENT_SECRET: &str = "secret";

pub const THEME_NAME_DEFAULT: &str = "default";

/* -- KEYVALS -- */
pub const KEYVAL_ENTER: u32 = 0xFF0D;
