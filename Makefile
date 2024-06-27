build:
	mkdir -p bin
	rm -f bin/*
	cargo b -r
	mv -f target/release/filler ./bin/
	mv -f target/release/visualizer ./bin/