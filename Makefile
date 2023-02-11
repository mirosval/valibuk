
.PHONY: setup
setup:
	cargo install cargo-watch cargo-expand

.PHONY: dev
dev:
	cargo watch -c -x 'check --workspace' -x 'test --workspace'

.PHONY: expand
expand:
	cargo watch -c -x '+nightly expand --example validated_macro'

