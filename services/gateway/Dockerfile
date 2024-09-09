FROM rust:1.80 as builder

WORKDIR /saki

COPY Cargo.toml Cargo.lock ./
COPY services/gateway/Cargo.toml services/gateway/
COPY services/image-getter/Cargo.toml services/image-getter/
COPY services/database/Cargo.toml services/database/
COPY services/protos/Cargo.toml services/protos/

# This is done to cache dependencies
# TODO: Work out how I can only copy 
# in the deps needed for the current
# service. i.e gateway only needs 
# the proto package and itself
RUN for service in gateway database image-getter protos; do mkdir -p "services/$service/src" && echo "fn main() {}" > "services/$service/src/main.rs"; done
RUN cargo fetch
RUN for service in gateway database image-getter protos; do rm -rf services/gateway/src; done

COPY . .

RUN cargo build --release --bin gateway

FROM debian:bullseye-slim

WORKDIR /saki

COPY --from=builder /saki/target/release/gateway /saki/gateway

CMD ["/saki/gateway"]
