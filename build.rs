fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/blob/v1/blob.proto");

    tonic_prost_build::configure().compile_protos(&["proto/blob/v1/blob.proto"], &["proto"])?;

    Ok(())
}
