# Learning Rust

This repo contains my implementations of coding exercises while following [The Rust Book V2](https://doc.rust-lang.org/book/second-edition)

## Running Code

### On your host machine

* Go [rust up](https://rustup.rs/) to install Rust on your machine.
* Once you can run

    ```
    cargo --version
    ```

    you can `cd` into any `ch0...` directory and run

    ```
    cargo run
    ```

### In Docker

* [Install Docker](https://store.docker.com/search?type=edition&offering=community) for your platform.
* In the root of the repo run

    ```
    docker-compose run rust
    ```

    This will start a bash shell in the repo root in a container that has Rust installed already. Now you can the above steps as if you had rust on your dev machine!