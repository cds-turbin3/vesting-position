# Generate test-report/<slug>.md with pinned-SHA permalinks back to each test,
# then inject the reverse `// Report:` back-links. Run from babelfish-tests/.
# Commit tests first: the permalink pins to HEAD, so tests/ must be clean.
SUBDIR := babelfish-tests

.PHONY: test-report
test-report:
	@git diff --quiet -- tests || { echo "commit changes under tests/ first (permalink pins to HEAD)"; exit 1; }
	@sha=$$(git rev-parse HEAD); \
	url=$$(git remote get-url origin | sed -E 's#git@github.com:#https://github.com/#; s#\.git$$##'); \
	base="$$url/blob/$$sha/$(SUBDIR)/"; \
	echo "pinning source links to $$base"; \
	rm -rf test-report && mkdir -p test-report; \
	FROOD_LINK_REPORT_DIR="$$(pwd)/test-report" FROOD_SOURCE_BASE="$$base" cargo test; \
	FROOD_SOURCE_BASE="$$base" cargo run --quiet --bin inject_backlinks
	@echo "done: review test-report/*.md and the // Report: edits, then commit"
