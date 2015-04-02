CC = cargo

.PHONY: rdm install clean

rdm:
	$(CC) build --release

install: rdm
	$(shell sudo mkdir -p /usr/share/rdm/themes/default/)
	$(shell sudo cp theme/background.png /usr/share/rdm/themes/default/)
	$(shell sudo cp theme/rdm.theme /usr/share/rdm/themes/default/)
	$(shell sudo cp -rf target/release/rdm /usr/local/bin)

clean:
	$(CC) clean
