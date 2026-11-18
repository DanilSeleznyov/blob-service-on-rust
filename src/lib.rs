#[path = "../internal/mod.rs"]
pub mod internal;

pub mod proto {
    tonic::include_proto!("blob.v1");
}
