# Build stage
FROM ekidd/rust-musl-builder AS build

WORKDIR /app

COPY src ./src/
COPY Cargo.toml ./

RUN cargo build --release

# Final stage
FROM gcr.io/distroless/cc

LABEL maintainer="Reyhan Kamil <reyhank95@hotmail.com>"

ENV TZ=Asia/Jakarta

WORKDIR /bin

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/rey_webapp ./
COPY www ./www/

EXPOSE 22333

CMD ["/bin/rey_webapp"]