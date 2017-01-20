use std::io::Write;

use dbus::{BusType, Connection, ConnectionItem, Error, Message, MessageItem, NameFlag};
use dbus::obj::{Argument, Interface, Method, ObjectPath, Property};

use constants::*;

pub struct Manager {
    running: bool
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            running:         false
        }
    }

    pub fn start(self) {
        let conn = match Connection::get_private(BusType::System) {
            Ok(c)   => c,
            Err(e)  => panic!("Manager: Failed to get DBUS connection: {:?}", e)
        };
        log_info!("Manager: Opened {:?}", conn);

        conn.register_name(DBUS_SERVICE_NAME, NameFlag::ReplaceExisting as u32).unwrap();
        log_info!("Manager: Registered service name {}", DBUS_SERVICE_NAME);

        let root_iface = Interface::new(
            vec!(Method::new(
                "Hello",
                vec!(),
                vec!(Argument::new("reply", "s")),
                Box::new(|msg| Ok(vec!(MessageItem::Str(format!("Hello {}!", msg.sender().unwrap())))))
            )),
            vec!(),
            vec!()
        );
        let mut root_path = ObjectPath::new(&conn, "/", true);
        root_path.insert_interface(DBUS_ROOT_PATH, root_iface);
        root_path.set_registered(true).unwrap();
        log_info!("Manager: Registered interface!");

        log_info!("Manager: Starting main loop!");
        for n in conn.iter(1) {
            match n {
                ConnectionItem::MethodCall(mut m) => {
                    if root_path.handle_message(&mut m).is_none() {
                        conn.send(Message::new_error(&m, "org.freedesktop.DBus.Error.Failed", "Object path not found").unwrap()).unwrap();
                        log_info!("Path not found");
                    }
                    else {
                        log_info!("Handled method call!");
                    }
                },
                _ => {},
            }
        }
        log_info!("Manager: Quit main loop. Exiting..");
    }
}
