# ================================================
# cargo-chef template
# ================================================
FROM rust:latest as chef
WORKDIR /poll
# install neccessary dependencies
RUN apt-get clean \
&& apt-get update -y \
&& apt-get install -y lld clang \
&& cargo install cargo-chef \
&& update-ca-certificates
# Create appuser
ENV USER=poll
ENV UID=10001
RUN adduser \
--disabled-password \
--gecos "" \
--home "/nonexistent" \
--shell "/sbin/nologin" \
--no-create-home \
--uid "${UID}" \
"${USER}"

# ================================================
# cargo-chef prepare
# ================================================
FROM chef as planner
COPY . .
# compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

# ================================================
# build project
# ================================================
FROM chef as builder
COPY --from=planner /poll/recipe.json recipe.json
# build our project dependencies, not our application!
# after this point, if our dependency tree stays the same, all layers should be cached.
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin poll

# ================================================
# live image
# ================================================
FROM debian:bullseye-slim AS runtime
WORKDIR /poll
RUN apt-get update -y \
# get dependencies
&& apt-get install -y --no-install-recommends openssl ca-certificates \
# clean up
&& apt-get autoremove -y \
&& apt-get clean -y \
&& rm -rf /var/lib/apt/lists/*
# import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /poll/target/release/poll poll
COPY --from=builder /poll/conf conf
ENV POLL_RUN_MODE production
USER poll:poll
ENTRYPOINT ["./poll"]
