extern crate serde;
extern crate schemafy;
extern crate serde_json;

use serde::{Serialize, Deserialize};
use serde_json::Error;

include!("./fhir-generated.rs");


fn main() -> Result<(),Error> {
    let practitioner: Practitioner = serde_json::from_str(r#"{"resourceType":"Practitioner","id":"e4d9b3e5-d6f8-54e5-940a-2ed35805c2f5","meta":{"lastUpdated":"2023-02-26T00:00:00Z","profile":["http://hl7.org/fhir/us/davinci-pdex-plan-net/StructureDefinition/plannet-Practitioner"]},"text":{"status":"generated","div":"<div xmlns=\"http://www.w3.org/1999/xhtml\">Claudio A Gomez</div>"},"identifier":[{"system":"http://hl7.org/fhir/sid/us-npi","value":"1760696769"}],"active":true,"name":[{"family":"Gomez","given":["Claudio"]}],"telecom":[{"extension":[{"url":"http://hl7.org/fhir/us/davinci-pdex-plan-net/StructureDefinition/via-intermediary","id":"3ffd2471-6d23-5e42-9fe5-54e9ecde0d31","valueReference":{"reference":"Location/3ffd2471-6d23-5e42-9fe5-54e9ecde0d31"}},{"url":"http://hl7.org/fhir/us/davinci-pdex-plan-net/StructureDefinition/via-intermediary","id":"57ba1b8d-0243-5b5c-95d3-eab0bfec181a","valueReference":{"reference":"Location/57ba1b8d-0243-5b5c-95d3-eab0bfec181a"}},{"url":"http://hl7.org/fhir/us/davinci-pdex-plan-net/StructureDefinition/via-intermediary","id":"b01913ae-dfa7-5952-b784-545ac07a36f6","valueReference":{"reference":"Location/b01913ae-dfa7-5952-b784-545ac07a36f6"}}]}],"gender":"male","communication":[{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/iso639-3","code":"ara"}],"text":"ara"},{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/iso639-3","code":"eng"}],"text":"eng"},{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/iso639-3","code":"ita"}],"text":"ita"},{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/iso639-3","code":"por"}],"text":"por"},{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/iso639-3","code":"spa"}],"text":"spa"}]}"#)?;
    println!("{:?}",practitioner);
    Ok(())
}