#proto_dir := "./src/api/generated"
source_proto_dir := "./crates/zitadel-gen/proto/"
# temp dirs for git clones of proto def
temp_zitadel_dir := "/tmp/zitadel"
temp_googleapis_dir := "/tmp/googleapis"
openapiv2_dir := source_proto_dir + "/protoc-gen-openapiv2/options"
validate_dir := source_proto_dir + "/validate"

zitadel_proto_version := "v2.69.1"


default: clean create-dirs googleapis-fetch-protos protoc_gen_openapiv2-fetch-protos buf_valitator_fetch zitadel-fetch-protos

generate-grpc: create-dirs googleapis-fetch-protos protoc_gen_openapiv2-fetch-protos buf_valitator_fetch zitadel-fetch-protos

clean:
    @rm -rf {{source_proto_dir}}

#generate-grpc:
#    buf generate https://github.com/zitadel/zitadel.git --path ./proto/zitadel
#
#install-tools:
#    cargo install protoc-gen-prost protoc-gen-tonic protoc-gen-prost-crate protoc-gen-prost-serde

create-dirs:
    mkdir -p {{source_proto_dir}}
    mkdir -p {{source_proto_dir}}/protoc_gen_openapiv2/options
    mkdir -p {{source_proto_dir}}/validate

googleapis-fetch-protos:
    git clone --depth 1 https://github.com/googleapis/googleapis.git {{temp_googleapis_dir}}
    mkdir -p {{source_proto_dir}}
    cp -r {{temp_googleapis_dir}}/google {{source_proto_dir}}
    rm -rf {{temp_googleapis_dir}}

zitadel-fetch-protos:
    git clone --depth 1 --branch {{zitadel_proto_version}} https://github.com/zitadel/zitadel.git {{temp_zitadel_dir}}
    mkdir -p {{source_proto_dir}}
    cp -r {{temp_zitadel_dir}}/proto/zitadel {{source_proto_dir}}
    rm -rf {{temp_zitadel_dir}}

buf_valitator_fetch:
    curl -L https://raw.githubusercontent.com/bufbuild/protoc-gen-validate/refs/heads/main/validate/validate.proto -o {{validate_dir}}/validate.proto

protoc_gen_openapiv2-fetch-protos:
    curl -L https://raw.githubusercontent.com/grpc-ecosystem/grpc-gateway/refs/heads/main/protoc-gen-openapiv2/options/annotations.proto -o {{openapiv2_dir}}/annotations.proto
    curl -L https://raw.githubusercontent.com/grpc-ecosystem/grpc-gateway/refs/heads/main/protoc-gen-openapiv2/options/openapiv2.proto -o {{openapiv2_dir}}/openapiv2.proto

all_examples:
    # fetch_profile_with_pat
    cargo run -p zitadel-examples --example fetch_profile_with_pat \
      --features "zitadel/api zitadel/interceptors tokio" --release

    # fetch_profile_with_service_account
    cargo run -p zitadel-examples --example fetch_profile_with_service_account \
      --features "zitadel/api zitadel/interceptors tokio" --release

    # actix_webapi_oauth_interception_basic
    cargo run -p zitadel-examples --example actix_webapi_oauth_interception_basic \
      --features "actix" --release

    # axum_webapi_oauth_interception_basic
    cargo run -p zitadel-examples --example axum_webapi_oauth_interception_basic \
      --features "axum" --release

    # rocket_webapi_oauth_interception_basic
    cargo run -p zitadel-examples --example rocket_webapi_oauth_interception_basic \
      --features "rocket" --release

    # rocket_webapi_oauth_interception_jwtprofile_cached
    cargo run -p zitadel-examples --example rocket_webapi_oauth_interception_jwtprofile_cached \
      --features "rocket zitadel/introspection_cache" --release

    # rocket_webapi_oauth_interception_jwtprofile
    cargo run -p zitadel-examples --example rocket_webapi_oauth_interception_jwtprofile \
      --features "rocket" --release

    # service_account_authentication
    cargo run -p zitadel-examples --example service_account_authentication --features "zitadel/credentials tokio" --release