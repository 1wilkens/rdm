.PHONY: install clean check clippy

CC = cargo

all: dbg

dbg:
	$(CC) build

check:
	$(CC) check

clippy:
	$(CC) clippy

run: dbg
	$(shell sudo target/debug/rdm)

rdm:
	$(CC) build --release

install: #rdm
# Copy themes
	$(shell sudo mkdir -p /usr/share/rdm/themes/default/)
	$(shell sudo cp -f theme/background.png /usr/share/rdm/themes/default/)
	$(shell sudo cp -f theme/rdm.theme /usr/share/rdm/themes/default/)
# Copy PAM file
	$(shell sudo cp -f data/rdm.pam /etc/pam.d/rdm)
# Copy systemd service file
	$(shell sudo cp -f data/rdm.service /usr/lib/systemd/system)
# Copy dbus file (not required for now)
	#$(shell sudo cp -f data/rdm.dbus /etc/dbus-1/system.d/rdm.conf)
# Copy binary
	$(shell sudo cp -rf target/release/rdm /usr/local/bin)

clean:
	$(CC) clean
