FROM rust:latest AS builder

WORKDIR /usr/src/app

RUN git config --global core.symlinks false


COPY backend ./

RUN cargo build --release

FROM debian:latest

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/solverevlos /app/solverevlos

EXPOSE 8080

CMD ["./solverevlos"]
