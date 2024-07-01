build: prepare install_deps compile

prepare:
	mkdir -p bin
	rm -f bin/*

install_deps:
ifeq ($(d), 1)
	rustup default stable
	apt update
	apt install -y build-essential libasound2-dev libudev-dev
endif

compile:
	cargo b -r
	mv -f target/release/filler ./bin/
	mv -f target/release/visualizer ./bin/
