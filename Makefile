.SILENT:
.PHONY: version setup

version: ##	Get the current project version.
version: ##		USAGE: make version
version:
	@grep -Po '\b^version\s*=\s*"\K.*?(?=")' Cargo.toml | head -1

setup: ##		Install all cargo extension. 
setup: ##			USAGE: make setup
	@cargo install cargo-edit
