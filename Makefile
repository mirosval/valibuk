
.PHONY: dev
dev:
	cargo watch --ignore wip -c -x 'check --workspace' -x 'test --workspace'

.PHONY: expand
expand:
	nix develop '.#nightly' --command \
		bash -c \
		"cargo watch --ignore wip -c -x 'expand --test integration_test'"

.PHONY: trybuild-overwrite
trybuild-overwrite:
	TRYBUILD=overwrite cargo test --test integration_test

.PHONY: publish
publish:
	cargo publish --manifest-path valibuk_core/Cargo.toml
	cargo publish --manifest-path valibuk_derive/Cargo.toml
	cargo publish 

.PHONY: build
build:
	cargo build

.PHONY: test
test:
	cargo test


.PHONY: docker-test
docker-test: 
	docker run \
		--rm \
		-it \
		-v $(shell pwd):/usr/src/valibuk \
		-w /usr/src/valibuk \
		#-e TRYBUILD=overwrite \
		rust \
		cargo test
