# Variables for commonly used paths and values
webdriver_path := "path/to/webdriver"
tauri_app_path := "path/to/tauri/app"
tauri_driver_path := "path/to/tauri/driver"

# Initializes the build environment
init:
    cargo run --bin init_build -q

# Downloads the required webdriver
download-webdriver: init _download-webdriver

_download-webdriver:
    cargo run --bin download_webdriver -q

# Builds the end-to-end tests
build-e2e:
    npm -w ck3oop-ui/tests-e2e run build --silent

# Runs the end-to-end tests
@test-e2e: init npm-install npm-build
    #!/bin/bash
    set -euox pipefail
    webdriver=$(just _download-webdriver)
    tauri_app=$(just _build-ui)
    merged_json=$(echo "$webdriver $tauri_app" | jq -s 'add')
    arguments=$(echo $merged_json | jq -r '
        "--tauri-app-path \(.executable)
        --tauri-driver-path \(.tauri_driver_binary)
        --webdriver-path \(.webdriver_binary)"'
    )
    npm run tests-e2e -- -- $arguments

# Installs npm dependencies
npm-install:
    npm install --silent

# Builds the npm project
npm-build:
    npm run build --silent

# Builds the UI
_build-ui:
    #!/bin/bash
    set -euox pipefail

    npm run tauri build -- -- --bundles=none -- \
    --message-format=json | tail -n2 | head -n1 \
    | jq -r '{"executable"}'

@build-ui: npm-install _build-ui

# Builds the Rust scripts
build-scripts:
    cargo build --package ck3oop-scripts

# Generates shell completion scripts
completion:
    . <(just --completions bash)

# Runs clippy to catch common mistakes and improve code
clippy:
    cargo clippy -- -D warnings

# Automatically applies clippy suggestions
clippy-fix extra="":
    cargo clippy --fix {{extra}} -- -D warnings

# Formats the code according to Rust's style guidelines
fmt:
    cargo fmt --

# Marks the latest release as the latest version
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
