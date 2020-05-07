FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/conduit
COPY . .
RUN cargo install --path .

# Container image that runs your code
FROM debian:stable-slim

RUN apt-get update && apt-get install -y \
    --no-install-recommends \
    openssl \
    libssl-dev
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*


# Copies your code file from your action repository to the filesystem path `/` of the container
COPY entrypoint.sh /entrypoint.sh

COPY --from=builder /usr/local/cargo/bin/conduit /app/conduit

# Code file to execute when the docker container starts up (`entrypoint.sh`)
ENTRYPOINT ["/entrypoint.sh"]
CMD ["/conduit"]