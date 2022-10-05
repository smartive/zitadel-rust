#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/caos/zitadel/main/console/src/favicon.ico",
    html_logo_url = "https://raw.githubusercontent.com/caos/zitadel/main/console/src/assets/images/zitadel-logo-solo-light.svg",
    issue_tracker_base_url = "https://github.com/buehler/zitadel-rust/issues/"
)]

#[cfg(feature = "api")]
pub mod api;
#[cfg(feature = "credentials")]
pub mod credentials;
