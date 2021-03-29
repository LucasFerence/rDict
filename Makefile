install:
	cargo install --root /usr/local --path $(CURDIR)

uninstall:
	cargo uninstall --root /usr/local rdict
