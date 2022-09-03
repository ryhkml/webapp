# Build stage
FROM ekidd/rust-musl-builder AS build

WORKDIR /app

COPY src ./src/
COPY www ./www/
COPY Cargo.toml ./

RUN cargo build --release

# Final stage
FROM scratch

LABEL maintainer="Reyhan Kamil <reyhank95@hotmail.com>"

ENV PORT=22333

WORKDIR /bin

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/rey_webapp ./
COPY --from=build /app/www ./www/

EXPOSE ${PORT}

CMD ["/bin/rey_webapp"]