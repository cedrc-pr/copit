# Overview

[Copit](https://copit.cedric-prin.dev)!

You can create a chat room where only those with the password can join.

Perfect for sharing snippets of code or long links with colleagues or between devices.

This project had two objectives:

- to set up a [CI/CD pipeline](https://github.com/cedrc-pr/infra) arround it with my home server to learn about DevOps
- to give me some practice with WebSockets and Redis (all built using a Rust API and a React frontend).

Have a look at the `README.md` file in `be/` or `fe/` if you’d like to find out more.

# Deployment

It requires [docker](https://www.docker.com/products/docker-desktop/) (and the [Rust toolchain](https://rustup.rs/) to generate secrets, but it isn't mandatory).

```shell
cp be/.env.example be/.env
# you may want to change secrets
docker compose up -d
```

To change secrets:

```shell
cd be/
cargo run --bin generate_secrets
# then copy/paste them
```
