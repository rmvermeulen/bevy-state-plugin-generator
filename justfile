test *flags:
    cargo-feature-combinations {{flags}} insta test --unreferenced warn

test-all: test

test-fast: (test "--fail-fast")
