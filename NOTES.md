## Architecture
- Multiprocess (Daemon <-> Greeter)
    - Communication via DBUS (or pipes)
    - React to session close -> show new greeter (DBUS ?)
- Main executable (`rdm`) launches Manager
    - -> Manager acquires DBUS name
    - -> Launches greeter in a new process
    - -> And starts event loop (quit via signals)
- One Greeter implementation initially (GTK based)

## Authentication
- Use new process for each pam authentication process as systemd maps pid to sessions/users => worker?
- Additional policy file required to start logind session? (`/etc/dbus-1/systemd./rdm.conf` ??)

## Theming
- GTK Greeter
    - Theme is GTK-.xml
    - Mandatory input fields identified via names
    - Single background image (initially) named `background.png`
- Other greeters? 

## Problems/Questions
- Signalhandling?? (Raw C API safe enough?)
- C Callback interaction
