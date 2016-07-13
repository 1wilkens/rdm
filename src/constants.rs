/* BASE CONSTANTS */
pub const APPLICATION_NAME        : &'static str = "rdm";

/* DBUS CONSTANTS */
pub const DBUS_SERVICE_NAME       : &'static str = "com.github.mrfloya.RDM";
pub const DBUS_ROOT_PATH          : &'static str = "com/github/mrfloya/RDM";

/* -- CONFIG SETTINGS -- */
pub const DEFAULT_CONFIG_FILE     : &'static str = "/etc/rdm.conf";

pub const DEFAULT_RUN_DIR         : &'static str = "/var/run/rdm";

pub const DEFAULT_SH_EXECUTABLE   : &'static str = "/bin/sh";
pub const DEFAULT_X_EXECUTABLE    : &'static str = "/usr/bin/X";
pub const DEFAULT_X_ARGS          : &'static str = "-logverbose";
pub const DEFAULT_X_DISPLAY       : &'static str = ":0";
pub const DEFAULT_X_VT            : &'static str = "vt01";

/* -- THEME SETTINGS -- */
pub const THEME_BASE_PATH         : &'static str = "/usr/share/rdm/themes/";

pub const THEME_MAIN_FILE_NAME    : &'static str = "rdm";
pub const THEME_MAIN_FILE_EXT     : &'static str = "theme";

pub const THEME_BACKGROUND_NAME   : &'static str = "background";
pub const THEME_BACKGROUND_EXT    : &'static str = "png";

pub const THEME_COMPONENT_WINDOW  : &'static str = "window";
pub const THEME_COMPONENT_BG      : &'static str = "background";
pub const THEME_COMPONENT_USER    : &'static str = "user";
pub const THEME_COMPONENT_SECRET  : &'static str = "password";

pub const THEME_NAME_DEFAULT      : &'static str = "default";

/* -- KEYVALS -- */
pub const KEYVAL_ENTER  : u32 = 0xFF0D;
