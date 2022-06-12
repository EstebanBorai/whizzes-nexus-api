<div align="center">
  <h1>nexus-api</h1>
  <h4 align="center">Self-hosteable social network</h4>
</div>

<div align="center">

![Build](https://github.com/whizzbit/nexus-api/workflows/build/badge.svg)
![Clippy](https://github.com/whizzbit/nexus-api/workflows/clippy/badge.svg)
![Deploy](https://github.com/whizzbit/nexus-api/workflows/deploy/badge.svg)
![Fmt](https://github.com/whizzbit/nexus-api/workflows/fmt/badge.svg)
![Test](https://github.com/whizzbit/nexus-api/workflows/test/badge.svg)

</div>

>  Nexus formal definition: a relationship or connection between people or things. [Source][5].

# Deployment

This application is published to a Heroku Dyno instance using the
[emk/heroku-buildpack-rust][6] on every push to `main` [throught this action][7].

# Development

## Requirements

- Rust and Cargo ([Rustup](https://rustup.rs))
- [SQLx CLI](#setup-sqlx-for-database-migrations)

## Getting Started

1. Clone this repository

```bash
git clone https://github.com/whizzbit/nexus-api.git
```

2. Create a copy of `.env.sample` in a new file with the name: `.env`

```bash
cp ./.env.sample ./.env
```

3. Execute Docker containers:


```bash
docker compose up
```

4. Run database migrations running `sqlx migrate run`. [You must complete SQLx Setup First](#setup-sqlx-for-database-migrations).

5. Open a new terminal session and run the server

```bash
cargo run
```

## Setup SQLx for Database Migrations

Install SQLx CLI using `cargo install` for PostgreSQL only

```bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

## GraphQL

The API exposed is build using async-graphql, which is a GraphQL implementation
build for Rust.

Visit the playground on [http://host:port/graphql][3], when running
the project locally.

> This GraphQL implementation uses the [Cursors Connections Pattern][2].

### The `DateTime` scalar

Our GraphQL gateway implements the `DateTime` scalar to specify date values.
You can read more on this scalar here: [DateTime][1].

# Contributing

Every kind of contribution to this project is welcome, please, don't hesitate
to open a Pull Request or Issue. I will be happy to help!

[1]: https://www.graphql-scalars.com/date-time/#only-date-time
[2]: https://relay.dev/graphql/connections.htm
[3]: http://0.0.0.0:7878/graphql
[4]: https://askubuntu.com/a/713442
[5]: https://www.merriam-webster.com/dictionary/nexus
[6]: https://github.com/emk/heroku-buildpack-rust.git
[7]: https://github.com/whizzbit/nexus-api/blob/main/.github/workflows/deploy.yml
