#[cfg(feature = "generate-protos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/proto") // Output directory for generated files
        .compile_protos(
            &[
                "milvus-proto/proto/milvus.proto",
                "milvus-proto/proto/common.proto",
                "milvus-proto/proto/schema.proto",
                "milvus-proto/proto/msg.proto",
                "milvus-proto/proto/rg.proto",
                "milvus-proto/proto/feder.proto"
            ],
            &["milvus-proto/proto"] // Include path for proto files
        )?;
    println!("Protos generated successfully");
    Ok(())
}

#[cfg(not(feature = "generate-protos"))]
fn main() {
    // Default main function when the feature is not enabled
    println!("Feature 'generate-protos' is not enabled.");
}
