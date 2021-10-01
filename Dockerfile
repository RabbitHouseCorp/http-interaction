FROM golang:1.17
WORKDIR /opt/postinteraction-docker


COPY . .

CMD ["go", "server.go"]