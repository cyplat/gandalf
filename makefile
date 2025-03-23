

SHELL := /bin/bash
.PHONY: help

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

clean-db: ## Remove and recreate app db
	docker kill gandalf-db
	docker rm gandalf-db
	docker volume rm gandalf_postgres_data
	docker compose up -d

clean: ## Clean the project using cargo
	cargo clean

build: ## Build the project using cargo
	cargo build

lint: ## Lint the project using cargo
	@rustup component add clippy
	cargo clippy

fmt: ## Format the project using cargo
	@rustup component add rustfmt
	cargo fmt

bump: ## Bump the version of the project
	@cargo bump patch

docs:
	cargo doc --open
