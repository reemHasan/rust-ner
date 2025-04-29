extern crate anyhow;

use std::env;
use rust_bert::pipelines::ner::NERModel;

fn main() -> anyhow::Result<()> {
    // Set-up model
    let ner_model = NERModel::new(Default::default())?;

    // Get input from command-line arguments
    let input: Vec<String> = env::args().skip(1).collect();

    // Check if input is provided
    if input.is_empty() {
        eprintln!("Error: Please provide input strings. Usage: cargo run -- <string1> <string2> ...");
        std::process::exit(1);
    }

    // Run model
    let output = ner_model.predict_full_entities(&input);
    for entity in output {
        println!("{entity:?}");
    }

    Ok(())
}