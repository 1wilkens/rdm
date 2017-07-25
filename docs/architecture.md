# architecture.md

## General

- Multiprocess (Daemon <-> Greeter)
    - Communication via unix socket in `/var/run/rdm/ipc.socket`
    - Library for greeter
        - Rust
        - C (wrapping Rust)
    - React to session close -> show new greeter (DBUS ?)
- Main executable (`rdm`) launches sub 'Managers'
    - SeatManager
    - DisplayManager
    - PowerManager?
- One Greeter implementation initially (GTK based)

## Authentication
- Use new process for each pam authentication process as systemd maps pid to sessions/users
    - daemon: get crendentials from greeter -> spawn session-helper
    - session-helpder: get credentials from daemon
        - authenticate against pam
        - spawn user session (if successful)
        - wait for user session to exit
        - cleanup and exit
- Additional policy file required to start logind session? (`/etc/dbus-1/systemd./rdm.conf` ??)

## Greeter
- GTK based
    - Theme is GTK-.xml
    - Mandatory input fields identified via names
    - Single background image (initially) named `background.png`
- Connects to socket

## Problems/Questions
- Signalhandling?? (Raw C API safe enough?)
- C Callback interaction
