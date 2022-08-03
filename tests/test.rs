use openapi_struct_gen::generate;

#[test]
fn test_generate() {
    generate(
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/example-schema.yaml"),
        concat!(env!("CARGO_TARGET_TMPDIR"), "/gen.rs"),
        &["Clone", "Serialize", "Deserialize"],
        &[("serde", "Serialize"), ("serde", "Deserialize")],
    )
    .unwrap();
}
