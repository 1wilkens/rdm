use slog::{Drain, Logger};
use slog_term::{FullFormat, PlainSyncDecorator};

pub fn plain_logger() -> Logger {
    let decorator = PlainSyncDecorator::new(::std::io::stdout());
    let drain = FullFormat::new(decorator).build().fuse();

    Logger::root(drain, o!())
}