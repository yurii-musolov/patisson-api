# syntax=docker/dockerfile:1
# check=error=true

FROM rust:1.86.0 AS build
WORKDIR /app
COPY . ./
# TODO: use target x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-gnu

FROM debian:12.10-slim AS setup
WORKDIR /app
COPY ./.docker/setup.sh ./
COPY --from=build /app/target/x86_64-unknown-linux-gnu/debug/auth ./
RUN apt-get update -y && apt-get install -y ca-certificates
ENTRYPOINT [ "bash" ]
CMD [ "./setup.sh" ]

FROM debian:12.10-slim AS auth
WORKDIR /app
COPY --from=build /app/target/x86_64-unknown-linux-gnu/debug/auth ./
RUN apt-get update -y && apt-get install -y ca-certificates
ENTRYPOINT [ "./auth" ]
CMD [ "serve" ]

FROM debian:12.10-slim AS provider
WORKDIR /app
COPY --from=build /app/target/x86_64-unknown-linux-gnu/debug/provider ./
RUN apt-get update -y && apt-get install -y ca-certificates
ENTRYPOINT [ "./provider" ]
CMD [ "serve" ]
