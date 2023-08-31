extern crate serde;
extern crate schemafy;
extern crate serde_json;

use serde::{Serialize, Deserialize};
use serde_json::Error;

mod model;

use model::Reference;


///
/// Parse a real resource JSON package for Identifier to validate parsing from JSON 

#[test]
fn identifier_type() {
    let identifier: Identifier = serde_json::from_str(r#"[{"system":"http://hl7.org/fhir/sid/us-npi","value":"1760696769"}]"#)?;
    assert_eq(identifier.value,"1760696769");
}