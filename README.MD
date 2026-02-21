⚠️ Do not forget to install the Cranelift codegen backend using

```bash
# install cranelift rust component
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
# install rust nightly locally
rustup override set nightly
```

## Prerequisites

You need to install `docker` engine, then pull the mariadb image

```bash
docker pull mariadb
```

### Libs

Lib to install ( use `aptitude search <lib>` to see packages that provide this lib)

> Actually we assume that you already have this lib installed by default.

- libssl.so.3
- libcrypto.so.3
- libgcc_s.so.1
- libm.so.6
- libc.so.6

### For dev

to improve the buildtime in development, we can use mold as linker

```bash
apt install mold
```

> If you don't want to use it, you just can remove or edit `.cargo/config.toml` file

## Development

### Start

We use in `.env.example` same datas used in `docker-compose.yml` so you casn start dev using :

```bash
cp .env.example .env
docker compose up -d
cargo run
```

To test endpoints api, you can open the `bruno` folder with `bruno` ( `Postman` opensource equivalent)

### You want to make improvments on this repo ?

Because we use the orm `sea-orm` you may need `sea-orm-cli`

```bash
cargo install sea-orm-cli
```

#### Edit the database structure

Generate a new entity file

```bash
sea-orm-cli generate entity
```

then you can edit the rs file and make edit on tables in methods down() and up().

Once you make the migration making

```bash
sea-orm-cli migrate
```

You can generate entities rs files with:

```bash
./scripts/generate_entities.sh
```

#### Code design

```bash
src
├── main.rs # Route file
├── modules
│   ├── auth
│   │   ├── domain.rs
│   │   ├── extractor.rs
│   │   ├── mod.rs
│   │   ├── route.rs
│   │   └── service.rs
│   ├── docker
│   │   ├── dto.rs
│   │   ├── mod.rs
│   │   ├── payload.rs
│   │   ├── route.rs
│   │   └── service.rs
│   ├── errors.rs
│   ├── models
│   │   ├── entities
│   │   └── mod.rs
│   ├── mod.rs
│   ├── responses.rs
│   ├── session
│   │   ├── domain.rs
│   │   ├── dto.rs
│   │   ├── mod.rs
│   │   └── service.rs
│   ├── states.rs
│   ├── types.rs
│   └── user
│       ├── dto.rs
│       ├── mod.rs
│       ├── payload.rs
│       ├── route.rs
│       └── service.rs
└── utils
    ├── cfg.rs
    ├── extractor.rs
    └── mod.rs
```

Actually, each module folder may have this type of files :

- `dto` <- The data struct sended to the client
- `payload` <- The data struct received from the client
- `domain` <- The data struct that is used internally and should not be exposed to the client
- `route` <- The file that returns routes related to the module
- `service` <- The service that handle the logic related to the module and return Services responses, compatible to ApiResponse
- `extractor` <- A tower extractor used by axum, permit to make an abstraction and use a sanityzed data in routes handler
