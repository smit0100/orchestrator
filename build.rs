fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/orchestrator/v1/orchestrator.proto")?;
    Ok(())
}