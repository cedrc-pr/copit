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

# Things that don't work

This project was more about WebSockets, Redis and specially about the DevOps around it.

That's why I didn't really take the time to correct bugs, but there are some:

- The frontend is not responsive at all on mobile. I don't think it is to hard to correct this because the frontend was made with also mobile in mind, and clearly seperated sections. Maybe, it is missing some colors...

- 'currently connected' users is incremented by 2 for all the other users each time a user in the room refresh the page. The solution is maybe to remove the Connect/Disconnect message and only resend the user count.

- 'is typing' may not works if the user leave/refresh the page will he is consider typing, because it's the frontend that sends the end of 'is typing'. The client should only be able to send a start 'is typing' frequently and the API send end of 'is typing' when the client stop sending them.

I didn't wrote issues about them for now, because, if I wrote them, I want them to be clean.
