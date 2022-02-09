FROM rust:1.58.1-bullseye as build-env
WORKDIR /app
ADD . /app
RUN RUSTFLAGS="-C link-arg=-s --cfg unsound_local_offset" cargo build --release

FROM gcr.io/distroless/cc-debian11:nonroot-amd64
ENV TZ="Asia/Jakarta"
WORKDIR /app
COPY --from=build-env /app/target/release/enma /app
ENTRYPOINT [ "./enma" ]