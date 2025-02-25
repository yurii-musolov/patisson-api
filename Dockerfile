# syntax=docker/dockerfile:1
# check=error=true

FROM rust:1.85.0 AS build
WORKDIR /app
COPY . ./
RUN cargo build --all-targets

FROM debian:12.9-slim AS auth
WORKDIR /app
COPY --from=build /app/target/debug/auth ./
ENTRYPOINT [ "./auth" ]
CMD [ "serve" ]

FROM debian:12.9-slim AS provider
WORKDIR /app
COPY --from=build /app/target/debug/provider ./
RUN apt-get update -y && apt-get install -y ca-certificates
ENTRYPOINT [ "./provider" ]
CMD [ "serve" ]
