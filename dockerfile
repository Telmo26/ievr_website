FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY backend/src ./src
COPY backend/Cargo.toml backend/Cargo.lock ./

RUN cargo build --release

FROM gcr.io/distroless/cc-debian13:latest

COPY ./backend/databases ./databases
COPY --from=builder /usr/src/app/target/release/ievr_backend .
CMD ["./ievr_backend"]
