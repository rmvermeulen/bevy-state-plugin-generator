# https://just.systems

# use nushell
set shell := ['nu', '-c']

# allow `{{ arg && "some value" }}` style syntax
set unstable
set lists

alias b := build
alias t := test

[private]
list:
    just --list

test-features *flags:
    cargo-feature-combinations {{ flags }} insta test

test: build test-features
test-fast: (test-features "--fail-fast")

# TODO: revisit before 2.0.0 where feature `rustfmt` is removed
# NOTE: this only works if `--all-features` implies "all test cases" and not "alternate test cases"
# check-snapshots action="reject":
#     cargo insta test --all-features --unreferenced {{action}}
# clean-snapshots: (check-snapshots "delete")

build:
    cargo-feature-combinations build
    just -f test-app/justfile build
