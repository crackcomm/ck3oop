init:
    cargo install cargo-run-bin@1.7.2

test-e2e:
    cargo run --bin test-e2e

completion:
    . <(just --completions bash)

clippy:
    cargo clippy -- -D warnings

clippy-fix extra="":
    cargo clippy --fix {{extra}} -- -D warnings

fmt:
    cargo fmt --

release-mark-latest:
    #!/bin/bash
    release_id=$(
      gh release list \
        --exclude-drafts --exclude-pre-releases \
        -L 20 --json tagName | \
        jq '.[] | select(.tagName | startswith("ck3oop-ui-v")) | .[]' -r | head -n1)
    # ask for confirmation
    read -p "Mark release $release_id as latest? [y/N] " -n 1 -r
    gh release edit $release_id --latest

#[confirm]
#cleanup-release-please:
#    gh release list  --json tagName | jq '.[].tagName' --raw-output0 | \
#    xargs -0 -I {} gh release delete "{}" --cleanup-tag
#
#    gh pr list --label "autorelease: tagged" --state=all  --json url \
#    | jq '.[].url' --raw-output0 \
#    | xargs -I {} -0 gh pr close {}
#
#    gh pr list --label "autorelease: pending" --state=all  --json url \
#    | jq '.[].url' --raw-output0 \
#    | xargs -I {} -0 gh pr close {}
#
#    gh pr list --label "autorelease: tagged" --state=all  --json url \
#    | jq '.[].url' --raw-output0 \
#    | xargs -I {} -0 gh pr edit --remove-label "autorelease: tagged" {}
#
#    gh pr list --label "autorelease: pending" --state=all  --json url \
#    | jq '.[].url' --raw-output0 \
#    | xargs -I {} -0 gh pr edit --remove-label "autorelease: pending" {}
