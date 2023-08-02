## Todo Rustup

### List installed components

    rustup component list


### Install rustup components

    rustup component add clippy rustfmt

#### Example usage

    cargo clippy
    cargo clippy -- -D warnings

    cargo fmt
    cargo fmt -- --check

### Install Cargo things

    cargo install cargo-watch cargo-audit

#### Example audit

    cargo audit


### Example watch

The commands are executed sequentially on success of previous.

    cargo watch -x check -x test -x run

### Cargo adding dependencies
    cargo add actix-web@4
    cargo add tokio@1

