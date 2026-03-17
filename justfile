proto_dir := "./src/api/generated"
zitadel_proto_version := "v2.71.19"

default: clean generate-grpc

clean:
    @rm -rf {{proto_dir}}

generate-grpc:
    buf generate https://github.com/zitadel/zitadel.git#tag={{zitadel_proto_version}} --path ./proto/zitadel

install-tools:
    cargo install protoc-gen-prost-crate cargo-edit