init:
    cargo install cargo-run-bin@1.7.2

download-webdriver:
    cargo run --bin download_webdriver -q

build-e2e:
    npm -w ck3oop-ui/tests-e2e run build --silent

@test-e2e: npm-install npm-build
    #!/bin/bash
    set -euox pipefail
    #just build-e2e
    webdriver=$(just download-webdriver)
    tauri_app=$(just build-ui)
    merged_json=$(echo "$webdriver $tauri_app" | jq -s 'add')
    arguments=$(echo $merged_json | jq -r '
        "--tauri-app-path \(.executable)
        --tauri-driver-path \(.tauri_driver_binary)
        --webdriver-path \(.webdriver_binary)"'
    )
    #node ck3oop-ui/tests-e2e/dist/main.js run $arguments

npm-install:
    npm install --silent

npm-build:
    npm run build --silent

@build-ui: npm-install
    #!/bin/bash
    set -euox pipefail

    npm run tauri build -- -- -- \
    --message-format=json | tail -n2 | head -n1 \
    | jq -r '{"executable"}'

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
