.PHONY: test

test:
	@./scripts/test-docs.sh
	@cargo test
