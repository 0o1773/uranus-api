FROM rust:latest as Builder

WORKDIR /app

COPY --link Cargo.toml ./
COPY --link build.rs ./
COPY --link src ./src
COPY --link migration ./migration
COPY --link proto ./proto

ARG DATABASE_URL

#RUN PB_REL="https://github.com/protocolbuffers/protobuf/releases" \
#    && curl -LO $PB_REL/download/v26.0/protoc-26.0-linux-x86_64.zip \
#    && unzip protoc-26.0-linux-x86_64.zip -d $HOME/.local \
#    && export PATH="$HOME/.local/bin:$PATH" \
#    && export PROTOC="$HOME/.local/bin/protoc"


RUN apt update && apt install -y protobuf-compiler libprotobuf-dev
RUN rustup component add rustfmt

RUN cargo install sea-orm-cli
RUN sea-orm-cli generate entity -u $DATABASE_URL -o src/entities
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12 as Runtime

COPY --from=Builder /app/target/release/uranus-grpc-server /app/uranus-grpc-server

ARG RIOT_API_KEY
ARG DATABASE_URL
ENV RIOT_API_KEY=$RIOT_API_KEY
ENV DATABASE_URL=$DATABASE_URL

EXPOSE 9000
CMD ["/app/uranus-grpc-server"]