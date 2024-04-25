fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/accounts.proto")?;
    tonic_build::compile_protos("proto/valorant.proto")?;
    Ok(())
}