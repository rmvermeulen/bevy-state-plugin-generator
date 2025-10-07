test *flags:
    cargo-feature-combinations {{flags}} insta test

test-all: test

test-fast: (test "--fail-fast")

# NOTE: this only works if `--all-features` implies "all test cases" and not "alternate test cases"
# check-snapshots action="reject":
#     cargo insta test --all-features --unreferenced {{action}}
# clean-snapshots: (check-snapshots "delete")

build:
    cargo-feature-combinations build
    just -f test-app/justfile build
