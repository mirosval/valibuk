
setup:
	cargo install cargo-watch cargo-expand

dev:
	#cargo watch -c -x '+nightly check --example simple'
	cargo watch -c -x '+nightly check --example without_macro'

expand:
	cargo watch -c -x '+nightly expand --example validated_macro'
