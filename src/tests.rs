#[test]
fn state() {
    let state_str = std::fs::read_to_string("data/docker/state.json").unwrap();
    let state = crate::TerraformParser::parse_state(&state_str).unwrap();

    println!("Outputs:");
    if let Some(outputs) = state.values.outputs {
        for (name, output) in outputs {
            println!("Output: {}", name);
            println!("Value: {}", output.value);
            println!();
        }
    }

    println!("Root module:");
    println!("Address: {:?}\n", state.values.root_module.address);

    println!("Resources:");
    for resource in state.values.root_module.resources {
        println!("Resource: {}", resource.address);
        println!("Name: {}", resource.name);
        println!();
    }
}
