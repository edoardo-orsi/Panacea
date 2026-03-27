.PHONY: up down logs build test check fmt db-shell nats-sub

up:
	docker compose up -d

down:
	docker compose down

logs:
	docker compose logs -f

build:
	cargo build --workspace

test:
	cargo test --workspace

check:
	cargo fmt --check
	cargo clippy --workspace --all-targets -- -D warnings

fmt:
	cargo fmt --workspace

db-shell:
	psql postgresql://panacea:panacea@localhost:5432/panacea

nats-sub:
	nats sub 'panacea.>'
