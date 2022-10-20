CROSS = ${HOME}/.cargo/bin/cross

TARGETS = 	aarch64-linux-android \
			x86_64-unknown-linux-gnu \
			x86_64-pc-windows-gnu
CROSS_TARGETS =	x86_64-pc-windows-msvc

all: cargo-build

release:
	mkdir release
	
cargo-build: release
	for TARGET in $(TARGETS); do \
		cargo build --release --target $$TARGET && \
		tar cz target/$$TARGET/release/basehan* -f release/$$TARGET.tar.gz; \
	done
	for TARGET in $(CROSS_TARGETS); do \
		$(CROSS) build --release --target $$TARGET && \
		tar cz target/$$TARGET/release/basehan* -f release/$$TARGET.tar.gz; \
	done
clean:
	rm -rf release
	rm -rf target

.PHONY: all clean tar cargo-build