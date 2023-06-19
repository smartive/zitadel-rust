proto_dir := "./src/api/generated"

default: clean generate-grpc

clean:
    @rm -rf {{proto_dir}}

generate-grpc:
    buf generate https://github.com/zitadel/zitadel.git --path ./proto/zitadel
