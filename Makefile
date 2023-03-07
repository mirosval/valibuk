
.PHONY: setup
setup:
	cargo install cargo-watch cargo-expand

.PHONY: dev
dev:
	cargo watch --ignore wip -c -x 'check --workspace' -x 'test --workspace'

.PHONY: expand
expand:
	cargo watch --ignore wip -c -x '+nightly expand --test integration_test'

.PHONY: trybuild-overwrite
trybuild-overwrite:
	TRYBUILD=overwrite cargo test --test integration_test

.PHONY: publish
publish:
	cargo publish --manifest-path valibuk_core/Cargo.toml
	cargo publish --manifest-path valibuk_derive/Cargo.toml
	cargo publish 
