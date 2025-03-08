SHELL := /bin/bash
.PHONY: help

help: ## Show command list.
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

format: ## Format project.
	@cargo fmt --quiet

lint: ## Lint project.
	@cargo clippy --quiet

test: ## Test project.
	@cargo test --quiet --workspace

env: ## Set environment variables.
	@. ./env.sh
	@echo "Environment variables are set."

run: ## Run project in Docker containers.
	@. ./env.sh &&	docker compose up -d --build --remove-orphans

all: format lint test run ## Run commands: format, lint, test, run.
	@echo "Formatting, linting, testing and running project in Docker containers completed."
