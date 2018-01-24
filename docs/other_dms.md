# other_dms.md

## lightdm

### General
- Design: https://www.freedesktop.org/wiki/Software/LightDM/Design/L
- Provides DBUS service
- Aquires seats via logind (DBUS) exclusively?!

### Hierarchy
- main.c
    - display\_manager\_new
        - seat_start
            - seat\_real\_start
                - display\_server\_start


## SDDM

### General
- Provides DBUS service

## Hierarchy
- DaemonApp
	- SeatManager
		- Seat
			- Display
				- DisplayServer

## GDM

### General

### Daemon - `main`
- block sigusr1
- set locale?
- parse options
- check root
- init logging
- init settings
- lookup user
	- names from settings
- ensure dirs
	- sets up `/var/run/gdm`, `/var/log/gdm`
- connects DBUS (`bus_reconnect`)
- creates `g_main_loop`
	- SIGTERM, SIGINT -> `on_shutdown_signal_cb`
	- SIGHUP -> `on_sighup_cb`
- runs main loop
- < RUNNING >
- shutdown settings
- shutdown log

### Daemon - `bus_reconnect`
- get system bus
- own name
	- `on_name_acquired` -> spin up manager object, set `show_local_greeter` and `xdmcp_enabled` and start the manager
	- `on_name_lost` -> cleanup manager instance and setup reconnect in 3 seconds
