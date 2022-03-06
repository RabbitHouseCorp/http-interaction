FROM rust:latest

WORKDIR /opt

COPY . .

RUN cargo build

EXPOSE 8080
CMD ["cargo", "run"]


