FROM ubuntu:22.04
COPY ./target/release/my-http-server-test ./my-http-server-test
ENTRYPOINT ["./my-http-server-test"]