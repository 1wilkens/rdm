use std::io::{self, Read, Write};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use dbus::{nonblock::Proxy, Path};
use dbus_tokio::connection;
use futures::prelude::*;
use slog::Logger;
use tokio_util::codec::Framed;

use rdmcommon::{ipc, util};

pub struct BusManager {
    log: Logger,
    conn: Arc<dbus::nonblock::SyncConnection>,
}

impl BusManager {
    pub fn new<L: Into<Option<Logger>>>(logger: L) -> Result<Self, Box<dyn std::error::Error>> {
        let log = logger.into().unwrap_or_else(util::plain_logger);

        debug!(log, "[BusManager::new] Connecting to dbus");
        // Connect to the D-Bus session bus (this is blocking, unfortunately).
        let (resource, conn) = connection::new_system_sync()?;

        // The resource is a task that should be spawned onto a tokio compatible
        // reactor ASAP. If the resource ever finishes, you lost connection to D-Bus.
        tokio::spawn(async {
            // FIXME: we should handle this gracefully
            let err = resource.await;
            panic!("[BusManager] Lost connection to D-Bus: {}", err);
        });

        Ok(BusManager { log, conn })
    }

    pub async fn run(&mut self) -> Result<(), ()> {
        let conn = self.conn.clone();
        let proxy = Proxy::new(
            "org.freedesktop.login1",
            "/org/freedesktop/login1",
            Duration::from_secs(2),
            conn,
        );
        // TODO: Handle timeouts and errors here
        let (res,): (Vec<(String, Path<'_>)>,) = proxy
            .method_call("org.freedesktop.login1.Manager", "ListSeats", ())
            .await
            .unwrap();
        println!("{:?}", res);
        for (x, y) in res {
            debug!(&self.log, "{:?},{:?}", x, y);
        }
        Ok(())
    }
}

impl Drop for BusManager {
    fn drop(&mut self) {
        debug!(&self.log, "Dropping BusManager");
    }
}
