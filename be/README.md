# Overview

This backend is a personal project designed to give me the opportunity to work on a Rust API (`tokio`, `poem`), with a focus on WebSockets and Redis, as well as a database (PostgreSQL), parsing (`validator`) and tracing (`tracing`).

I know that Redis is of no use in my case, but in a hypothetical scenario where I have thousands of users, the first bottleneck is the API. This Redis instance enables the application to work with multiple APIs.

# Folder structure

```
src/
    routes/
        users.rs
    adapters/       # an adapter interact with an external service (database, ect)
        postgres/
    tools/          # an internal service (offen used to encapsulate "complex" logic)
```

# conventions

- `unwrap()` forbidden (if not justified)
- every API input must be parsed
- promote structures with implementations
- real error handling (avoid `_ => ...`)

# Development setup

It requires [docker](https://www.docker.com/products/docker-desktop/) and the [Rust toolchain](https://rustup.rs/).

```shell
cp .env.example .env
docker compose up -d
cargo run
```

# WebSocket workflow

I explains it there because it has been a bit complicated for me.

A web socket is splited in two parts:

- the **stream** is what the web socket sends to the API
- the API sends something through its **sink**

There is no authentication, so, to distinguish users between connections, I create a unique ID which I associate with an UnboundedSender (the only owner of the sink) using a HashMap, and the stream is read in a loop while the connection remains open.

Here is an example, A and B are in the same Room, A sends a message :

- the stream of A is red (and parsed)
- it publish the adapted event to the Redis
- it is also subscribe to those events so it received the event
- then gather all users (their ids) in the concerned room
- for all of those ids, it send the event by their UnboundedSender
  - each UnboundedSender is its own asynchronous task
  - the task wait in a loop indefinitely for a new message
  - if one is received, it sends it to the sink
  - in this example, the sink of A and B
- so it's send to A and B
