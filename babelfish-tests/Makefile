# Generate test-report/<slug>.md with pinned-SHA permalinks back to each test,
# then inject the reverse `// Report:` back-links. Run from babelfish-tests/.
# Commit tests first: the permalink pins to HEAD, so tests/ must be clean.
SUBDIR := babelfish-tests

# The flagship narratives, mirrored to ../md-reports-babelfish/ so they sit
# beside the anchor-litesvm suite's ../md-reports/ for side-by-side reading.
# The full per-test set stays in test-report/; this list is the curated cut,
# matched one-for-one to the three stories md-reports/ tells.
FLAGSHIPS := full_lifecycle claims_at_linear_checkpoints vested_tokens_are_forfeited_past_the_grace_window

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
	@mkdir -p ../md-reports-babelfish
	@for f in $(FLAGSHIPS); do cp "test-report/$$f.md" ../md-reports-babelfish/; done
	@echo "done: review test-report/*.md, ../md-reports-babelfish/, and the // Report: edits, then commit"
