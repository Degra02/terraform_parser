#[test]
fn state() {
    let state_str = std::fs::read_to_string("data/docker/state.json").unwrap();
    let state = crate::TerraformParser::parse_state(&state_str).unwrap();

    println!("Outputs:\n\n");
    if let Some(outputs) = state.values.outputs {
        for (name, output) in outputs {
            println!("Output: {}", name);
            println!("Value: {}", output.value);
            println!("Sensitive: {}", output.sensitive);
        }
    }

    println!("Root module:\n\n");
    println!("Address: {:?}", state.values.root_module.address);
    for resource in state.values.root_module.resources {
        println!("Resource: {}", resource.address);
        println!("Mode: {:?}", resource.mode);
        println!("Type: {}", resource.resource_type);
        println!("Name: {}", resource.name);
        println!("Index: {:?}", resource.index);
        println!("Provider: {}", resource.provider_name);
        println!("Schema version: {}", resource.schema_version);
        println!("Values: {:?}", resource.values);
        println!("Sensitive values: {:?}", resource.sensitive_values);
        println!();
    }
}
