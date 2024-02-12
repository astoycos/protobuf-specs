/// Find the standard protobuf include directory.
fn protobuf_include_path() -> String {
    let mut protobuf_root = which::which("protoc")
        .ok()
        // dirname(/bin/protoc) / ../
        .and_then(|path| path.ancestors().nth(2).map(|p| p.to_path_buf()))
        .expect("protobuf installation directory not found!");
    protobuf_root.push("include");
    protobuf_root.to_str().unwrap().to_owned()
}

fn main() -> anyhow::Result<()> {
    let includes = vec![
        concat!(env!("CARGO_MANIFEST_DIR"), "/../../../protos").to_owned(),
        // WKTs path
        protobuf_include_path(),
        // googleapi types path
        std::env::var("SIGSTORE_PROTOBUF_EXTRA_INCLUDE").unwrap_or("/opt/include".to_owned()),
    ];

    let mut config = prost_build::Config::new();
    config
        .include_file("mod.rs")
        .type_attribute(
            ".",
            "#[derive(derive::Deserialize_proto, derive::Serialize_proto)]",
        )
        // Disable problematic comments interpreted as doctests.
        .disable_comments([".io.intoto.Envelope"]);

    prost_reflect_build::Builder::new()
        .file_descriptor_set_bytes("crate::FILE_DESCRIPTOR_SET_BYTES")
        .compile_protos_with_config(
            config,
            &glob::glob(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../protos/*.proto"
            ))
            .expect("no protos found!")
            .flatten()
            .collect::<Vec<_>>(),
            &includes,
        )?;

    Ok(())
}