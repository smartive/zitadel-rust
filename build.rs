const INCLUDES: &[&str; 4] = &[
    "external/zitadel/proto",
    "external/protoc-gen-validate",
    "external/googleapis",
    "external/grpc-gateway",
];

const CLIENT_PROTOS: &[&str; 3] = &[
    "zitadel/admin.proto",
    "zitadel/auth.proto",
    "zitadel/management.proto",
];

const NON_CLIENT_PROTOS: &[&str; 19] = &[
    "protoc-gen-openapiv2/options/annotations.proto",
    "protoc-gen-openapiv2/options/openapiv2.proto",
    "validate/validate.proto",
    "zitadel/action.proto",
    "zitadel/app.proto",
    "zitadel/auth_n_key.proto",
    "zitadel/change.proto",
    "zitadel/features.proto",
    "zitadel/idp.proto",
    "zitadel/member.proto",
    "zitadel/message.proto",
    "zitadel/metadata.proto",
    "zitadel/object.proto",
    "zitadel/options.proto",
    "zitadel/org.proto",
    "zitadel/policy.proto",
    "zitadel/project.proto",
    "zitadel/text.proto",
    "zitadel/user.proto",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(false)
        .compile(NON_CLIENT_PROTOS, INCLUDES)?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(CLIENT_PROTOS, INCLUDES)?;

    Ok(())
}
