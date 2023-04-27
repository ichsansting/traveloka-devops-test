FROM rust:1.69-slim

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["traveloka-devops-test"]