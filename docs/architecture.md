# **[rdm]** -- architecture.md

## General
- Multiprocess (Daemon <=> Greeter)
    - Communication via unix socket in `/var/run/rdm/ipc.socket`
    - Library for greeter
        - Rust
        - C (wrapping Rust)
    - React to session close => show new greeter (DBUS?)
- OOP (SDDM) vs flat architecture (lightdm)
    - Which one is more "rusty"?
    - OOP
        - Main executable (`rdm`) launches sub 'Managers'
            - SeatManager
            - DisplayManager
            - PowerManager?
        - [+] Clear-cut separation of concerns (easier to reason about?)
        - [--] More complexity (`Traits` vs `structs`)
    - flat
        - `main` sets up different features sequentially
            - Logging
            - IPC
            - (Dbus)
        - [+] All objects in scope
        - [+] Less complexity? (especially in regards to lifetimes)
        - [--] Long `main` function
- `Tokio` for IPC and event loop?
    - Does the daemon need a real event loop? (e.g. multiplex DBUS, greeter IPC and systemd)
- One Greeter implementation initially (`GTK` based)

## Authentication/Session management
- Use new process for each pam authentication process as systemd maps pid to sessions/users
    - daemon: get credentials from greeter => spawn session-helper
    - session-helper: get credentials from daemon
        - authenticate against pam
        - spawn user session (if successful)
        - wait for user session to exit
        - cleanup and exit
    - Credential flow: Greeter => Daemon => Session-helper (transient) => Session
    - Which process uses which pam file?
- Additional policy file required to start logind session? (`/etc/dbus-1/systemd./rdm.conf` ??)

## Greeter
- GTK based
    - Theme is GTK-.xml
    - Mandatory input fields identified via names
    - Single background image (initially) named `background.png`
- Connects to socket
- For more customization move credential closures into theme (taking `void*` as interface pointer?)
