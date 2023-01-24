# ================================================
# cargo-chef template
# ================================================
FROM rust:latest as chef
WORKDIR /poll
RUN cargo install cargo-chef

# ================================================
# cargo-chef prepare computes a lock-like file for
# our project
# ================================================
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ================================================
# build project
# ================================================
FROM chef as builder
# build our project dependencies, not our application!
COPY --from=planner /poll/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# after this point, if our dependency tree stays
# the same, all layers should be cached.
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin poll

# ================================================
# live image
# ================================================
FROM debian:bullseye-slim AS runtime
WORKDIR /poll
# get dependencies and clean up after we finish
RUN apt-get update -y \
&& apt-get install -y --no-install-recommends openssl ca-certificates \
&& apt-get autoremove -y \
&& apt-get clean -y \
&& rm -rf /var/lib/apt/lists/*
# import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /poll/target/release/poll poll
COPY --from=builder /poll/conf conf
COPY --from=builder /poll/static static
USER docker:docker
ENV POLL__RUN_MODE production
ENTRYPOINT ["./poll"]
