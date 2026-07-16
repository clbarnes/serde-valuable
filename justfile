_:
    just --list

fix:
    cargo fix --all --allow-dirty --allow-staged
    cargo fmt --all

check:
    cargo clippy --all-targets --all-features -- -D warnings
    cargo fmt --all -- --check

test:
    cargo test --all --all-features

release level:
    test -z "$(git status --porcelain)" || ( git status && false )
    changelog release {{level}}
    git add CHANGELOG.md
    git commit -m "Bump changelog to v$(changelog version --latest)"
    cargo release {{level}} --execute --push-remote clbarnes/main
