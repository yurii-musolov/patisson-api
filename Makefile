format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet --workspace

run:
	. ./env.sh &&	docker compose up -d --build --remove-orphans

all: format lint test run
