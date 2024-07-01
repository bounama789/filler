build: prepare install_deps compile

prepare:
	rustup update
	mkdir -p bin
	rm -f bin/*

install_deps:
ifeq ($(d), 1)
	apt update
	apt install -y build-essential 
endif

compile:
	cargo b -r
	mv -f target/release/filler ./bin/
	mv -f target/release/visualizer ./bin/
