## Architecture
- Multiprocess (Server <-> Greeter)
    - Communication via pipes (DBUS overkill?)
    - React to session close -> show new greeter (DBUS ?)
- One Greeter implementation initially (GTK based)

## Authentication
- Use new process for each pam authentication as systemd maps pid to sessions/users
-

## Theming
- GTK Greeter
    - Theme is GTK-.xml
    - Mandatory input fields identified via names
    - Single background image (initially) named 'background.png'
- Other greeters?
