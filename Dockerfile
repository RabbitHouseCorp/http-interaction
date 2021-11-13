FROM rust:latest

WORKDIR /opt

COPY . .

RUN cargo build

CMD ["cargo", "run"]


EXPOSE 3030
