.PHONY: report
# pass along to `cargo make` (see ./Makefile.toml)
%:
	cargo make $@

report:
	cargo make report

