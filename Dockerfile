# syntax=docker/dockerfile:1
# check=error=true

FROM rust:1.86.0 AS build
WORKDIR /app
COPY . ./
RUN cargo build --all-targets

FROM debian:12.10-slim AS setup
WORKDIR /app
COPY --from=build /app/target/debug/auth ./
COPY --from=build /app/target/debug/provider ./
COPY ./.docker/migrate.sh ./
# TODO: Use migration script './migrate.sh'
ENTRYPOINT [ "./auth" ]
CMD [ "migrate" ]

FROM debian:12.10-slim AS auth
WORKDIR /app
COPY --from=build /app/target/debug/auth ./
ENTRYPOINT [ "./auth" ]
CMD [ "serve" ]

FROM debian:12.10-slim AS provider
WORKDIR /app
COPY --from=build /app/target/debug/provider ./
RUN apt-get update -y && apt-get install -y ca-certificates
ENTRYPOINT [ "./provider" ]
CMD [ "serve" ]
