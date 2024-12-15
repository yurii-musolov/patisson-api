FROM rust:1.83.0 AS build
WORKDIR /app
COPY . ./
RUN cargo build --all-targets

FROM debian:12.8 AS auth
WORKDIR /app
COPY --from=build /app/target/debug/auth ./
ENTRYPOINT [ "./auth" ]
CMD [ "serve" ]
