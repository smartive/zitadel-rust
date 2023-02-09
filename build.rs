const INCLUDES: &[&str; 4] = &[
    "external/zitadel/proto",
    "external/protoc-gen-validate",
    "external/googleapis",
    "external/grpc-gateway",
];

const CLIENT_PROTOS: &[&str; 4] = &[
    "zitadel/admin.proto",
    "zitadel/auth.proto",
    "zitadel/management.proto",
    "zitadel/system.proto",
];

const NON_CLIENT_PROTOS: &[&str; 21] = &[
    "protoc-gen-openapiv2/options/annotations.proto",
    "protoc-gen-openapiv2/options/openapiv2.proto",
    "validate/validate.proto",
    "zitadel/action.proto",
    "zitadel/app.proto",
    "zitadel/auth_n_key.proto",
    "zitadel/change.proto",
    "zitadel/idp.proto",
    "zitadel/instance.proto",
    "zitadel/member.proto",
    "zitadel/message.proto",
    "zitadel/metadata.proto",
    "zitadel/object.proto",
    "zitadel/options.proto",
    "zitadel/org.proto",
    "zitadel/policy.proto",
    "zitadel/project.proto",
    "zitadel/settings.proto",
    "zitadel/text.proto",
    "zitadel/user.proto",
    "zitadel/v1.proto",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "api")]
    {
        println!("cargo:rerun-if-changed=external/zitadel/proto");
        println!("cargo:rerun-if-changed=external/protoc-gen-validate");
        println!("cargo:rerun-if-changed=external/googleapis");
        println!("cargo:rerun-if-changed=external/grpc-gateway");

        tonic_build::configure()
            .emit_rerun_if_changed(false)
            .build_server(false)
            .build_client(false)
            .compile(NON_CLIENT_PROTOS, INCLUDES)?;
        tonic_build::configure()
            .emit_rerun_if_changed(false)
            .build_server(false)
            .build_client(true)
            .compile(CLIENT_PROTOS, INCLUDES)?;
    }

    Ok(())
}
