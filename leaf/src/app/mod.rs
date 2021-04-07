pub mod dispatcher;
pub mod dns_client;
pub mod inbound;
pub mod nat_manager;
pub mod outbound;
pub mod router;

#[cfg(feature = "api")]
pub mod api;

#[cfg(any(
    target_os = "ios",
    target_os = "android",
    target_os = "macos",
    target_os = "linux"
))]
pub mod fake_dns;
