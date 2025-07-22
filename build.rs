fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::Config::new()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(&["proto/intentsource.proto"], &["proto/", "src/"])?;
    
    Ok(())
}