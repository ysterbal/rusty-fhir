fn main() {
    if cfg!(feature = "schema-generate") {
    let schema_path = "schemas/fhir.schema.json";
    schemafy_lib::Generator::builder()
        .with_root_name_str("Schema")
        .with_input_file(schema_path)
        .build()
        .generate_to_file("src/model.rs")
        .unwrap();
    }
}