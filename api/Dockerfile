FROM rustlang/rust:nightly as base

WORKDIR /app

# Install tools used in running the app in dev mode:
# * cargo-chef builds all dependencies separate from the app so they can be layer cached.
# * diesel_cli does stuff?
RUN cargo install cargo-chef --version 0.1.33 && \
    cargo install diesel_cli --no-default-features --features postgres && \
    rm -rf "$CARGO_HOME"/registry/

# In this layer, create a "recipe", a list of all our dependencies.
FROM base as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin hippo

# Extract the smallest possible image from what we built
FROM debian:bullseye-slim as runtime
WORKDIR /app
RUN apt-get update && \
    apt-get install -y --no-install-recommends tini libpq5 && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/hippo /usr/local/bin/hippo
COPY --from=base /usr/local/cargo/bin/diesel /usr/local/bin/diesel

EXPOSE 8888
VOLUME ["/usr/local/cargo"]

ENTRYPOINT ["tini", "--"]
CMD ["/usr/local/bin/hippo"]
