fn main() -> Result<(), Box<dyn std::error::Error>> {
    volo_build::Builder::protobuf()
        .include_dirs(vec![std::path::PathBuf::from("proto")])
        .filename("proto/syros.proto".into());
    Ok(())
}
