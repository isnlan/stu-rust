fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src")
        .build_server(true)
        .compile(
            &[
                "protos/message.proto",
            ],
            &["protos"],
        )?;
    Ok(())
}
