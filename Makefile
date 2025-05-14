.PHONY: all run test coverage clean

# Färgkoder för terminalen
GREEN=\033[0;32m
NC=\033[0m

all: run

run:
	@echo "$(GREEN)> Build and run the program...$(NC)"
	cargo run

test:
	@echo "$(GREEN)> running tests...$(NC)"
	cargo test

coverage:

coverage:
	cargo tarpaulin --out Html

clean:
	@echo "$(GREEN)> Cleaning...$(NC)"
	cargo clean

