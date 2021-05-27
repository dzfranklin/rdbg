use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = &[
        "code_view.proto",
        "recorder.proto",
        "replayer.proto",
        "theme.proto",
    ];
    let dir = PathBuf::from("../protos");
    let protos = protos.iter().map(|p| dir.join(p)).collect::<Vec<_>>();
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&protos, &[dir])?;
    Ok(())
}
