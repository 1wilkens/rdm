/// Some useful macros for logging

//TODO: Extract submacro

#[cfg(debug_assertions)]
macro_rules! log_debug(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), "[DBG] {}", format!($($arg)*)) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

#[cfg(not(debug_assertions))]
macro_rules! log_debug(
    ($($arg:tt)*) => (
        ()
    )
);

macro_rules! log_info(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), "[INF] {}", format!($($arg)*)) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

macro_rules! log_err(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), "[ERR] {}", format!($($arg)*)) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);
