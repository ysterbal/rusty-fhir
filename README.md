# rusty-fhir
Rust based FHIR server

# Notes 
To run the generate function (not working correctly generating some duplicate in struct names in FHIR there are _ concepts for internally referenced structures the generator does not deal with those nicely also)

```
cargo build --features=schema-generate
```
This will generate a model.rs file but it will have compile errors 

