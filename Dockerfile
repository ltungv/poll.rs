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
RUN adduser \
--disabled-password \
--no-create-home \
--gecos "" \
--shell "/sbin/nologin" \
--home "/nonexistent" \
--uid 10001 \
poll
USER poll:poll
# import from builder.
COPY --from=builder /poll/target/release/poll poll
COPY --from=builder /poll/conf conf
COPY --from=builder /poll/static static
ENV POLL__RUN_MODE production
ENTRYPOINT ["./poll"]
