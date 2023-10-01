export RUST_LOG_STYLE			?= debug
export RUST_LOG				?= debug
MAKEFILE_PATH				:= $(realpath $(firstword $(MAKEFILE_LIST)))
GIT_ROOT				:= $(shell dirname $(MAKEFILE_PATH))
INSTALL_PATH				:= $(HOME)/usr/libexec/
GREPRF_NAME				:= greprf
GREPRF_DEBUG_EXEC			:= target/debug/$(GREPRF_NAME)
GREPRF_RELEASE_EXEC			:= target/release/$(GREPRF_NAME)


all: release

release: purge $(GREPRF_RELEASE_EXEC)

run: clean
	cargo run plain

clean:
	@rm -rfv $(GREPRF_RELEASE_EXEC) $(GREPRF_DEBUG_EXEC)

purge: clean
	@rm -rf build

check fix test build:
	cargo $@ --all-targets

$(GREPRF_RELEASE_EXEC): $(INSTALL_PATH)
	cargo build --release

$(GREPRF_DEBUG_EXEC): $(INSTALL_PATH)
	cargo build

.PHONY:  test build run all purge
