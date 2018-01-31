# **[rdm]** -- architecture.md

## General
- Libraries/Technologies
    - `systemd`/`logind`
    - `DBUS`
        - Communication with `logind`
        - (Communication between greeter and daemon?)
- Multiprocess (Daemon <=> Greeter)
    - Communication via unix socket in `/var/run/rdm/ipc.socket`
    - Library for greeter
        - Rust
        - C (wrapping Rust)
    - React to session close => show new greeter (DBUS?)
- OOP (SDDM) vs flat architecture (lightdm, gdm)
    - Which one is more "rusty"?
    - OOP
        - Main executable (`rdm`) launches sub 'Managers'
            - SeatManager
            - DisplayManager
            - PowerManager?
        - [+] Clear-cut separation of concerns (easier to reason about?)
        - [--] More complexity (`Traits` vs `structs`)
        - Questions
            - How to link managers? (Callbacks?, Fn(Once) & friends) / How does this work with struct methods?
    - flat
        - `main` sets up different features sequentially
            - Logging
            - IPC
            - (Dbus)
        - [+] All objects in scope
        - [+] Less complexity? (especially in regards to lifetimes)
        - [--] Long `main` function
- `Tokio` for IPC and event loop?
    - Does the daemon need a real event loop? (e.g. multiplex DBUS, greeter IPC and systemd) - probably
- One Greeter implementation initially (`GTK` based)

## Daemon
- Responsibilities
    - Manage event loop with the following components
        - IPC socket
        - DBUS: Monitor logind
        - DBUS: Own API (maybe v2 and behind feature flag?)
        - Signals?
    - Spawn session helpers for successful logins
    - Monitor sessions and spawn new greeter after session ends

## Greeter
- Responsibilities
    - Obtain credentials from user
    - Feed credentials to daemon
    - Just exit or interact with session helper?
- GTK based
    - Theme is GTK-.xml
    - Mandatory input fields identified via names
    - Single background image (initially) named `background.png`
- Connects to socket
- For more customization move credential closures into theme (taking `void*` as interface pointer?)

## Authentication/Session management
- Use new process for each pam authentication process as `systemd` maps pid to sessions/users
    - daemon: get credentials from greeter => spawn session-helper
    - session-helper: get credentials from daemon
        - authenticate against pam
        - spawn user session (if successful)
        - wait for user session to exit
        - cleanup and exit
    - Credential flow: Greeter => Daemon => Session-helper (transient) => Session
    - Which process uses which pam file?
- Additional policy file required to start logind session? (`/etc/dbus-1/systemd./rdm.conf` ??)
