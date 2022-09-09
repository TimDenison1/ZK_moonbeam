# Purpose

The aim of this repo is to suggest a non-recursive version of the encryption available for the Moonbeam parachain.  
WIP. 

## Rust adoption

Have the latest version of rust avaiable installed. 
ZK_Moonbeam is built using latest stable [rust](https://www.rust-lang.org/), which
you should install via [rustup](https://rustup.rs/).

To install the remaining dependencies using a package manager, run one of the
following commands.

Homebrew (OS X):
```bash
brew install openssl zeromq pkg-config protobuf libpq
```

APT (Ubuntu):
```bash
apt install \
    build-essential \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    libsqlite3-dev \
    libpq-dev \
    libsasl2-dev \
    libxml2-dev \
    libzmq3-dev \
    openssl
```

Once you have the prerequisites installed, run `cargo build` from the root
directory. This command builds all of the Grid components, including `gridd`
(the ZK daemon), the CLI, and all of the smart contracts in the `contracts`
directory.

To build individual components, run `cargo build` in the component directories.
For example, to build only the grid-cli, navigate to `cli`, then run
`cargo build`.

### Building with Docker

To build Grid using Docker, run `docker-compose build` from the root directory.
This command builds Docker images for all of the Grid components, including
`gridd` (the grid daemon), the CLI, and all of the smart contracts in the
`contracts` directory.

To build individual components using Docker, run
`docker-compose build <component>` from the root directory. For example, to
build only the grid-cli, run `docker-compose build grid-cli`.

To use Docker to build Grid with experimental features enabled, set an
environment variable in your shell before running the build commands. For
example: `export 'CARGO_ARGS= --features experimental'`. To go back to
building with default features, unset the environment variable:
`unset CARGO_ARGS`

