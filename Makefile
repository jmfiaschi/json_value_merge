.SILENT:
.PHONY: version

version: ##	Get the current project version.
version: ##		USAGE: make version
version:
	@grep -Po '\b^version\s*=\s*"\K.*?(?=")' Cargo.toml | head -1
