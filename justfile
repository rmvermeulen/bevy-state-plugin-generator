test *flags:
    cargo-feature-combinations {{flags}} insta test

test-all: test

test-fast: (test "--fail-fast")

check-snapshots action="reject":
    cargo insta test --all-features --unreferenced {{action}}

clean-snapshots: (check-snapshots "delete")
