CRATE_NAME=go-installer

CC = rustc
CXX = rustc
CFLAGS = --crate-name $(CRATE_NAME) --edition 2018 -C debuginfo=2 -C lto -C incremental -C opt-level=3 -Os -C
link-args="-static"
LDFLAGS = -L.
LDLIBS =

all: $(CRATE_NAME)

$(CRATE_NAME): src/main.rs Cargo.toml .gitignore README.md
	@echo Building Rust project with Cargo
	cargo build --release

%.o: %.rs
	@echo Compiling $< to $@
	$(CC) $(CFLAGS) -c -o $@ $<

