### JSON GET request's of from the command line using Rust's `cargo` and optionally `jq`

## Download

    git clone https://github.com/jasonleonhard/get_request

## Enter

    cd get_request

## Run

    cargo run "https://jasonleonhard.com/pins/12"

This is currently written to allow GET requests of any arbitrary text, JSON or otherwise.

Simply avoid the `| jq` at the end of the command to treat as standard text.

## Optional

    brew install jq
    cargo run "https://jasonleonhard.com/pins/12" | jq

## Test

    cargo test
