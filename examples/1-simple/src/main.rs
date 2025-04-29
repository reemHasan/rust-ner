extern crate anyhow;

use rust_bert::pipelines::ner::NERModel;

fn main() -> anyhow::Result<()> {
    //    Set-up model
    let ner_model = NERModel::new(Default::default())?;

    //    Define input
    let input = [
        "My name is Reem and I was born in lattakia.",
        "In Syria the baklava is great.",
        "I used to go every day to beach",
    ];

    //    Run model
    let output = ner_model.predict_full_entities(&input);
    for entity in output {
        println!("{entity:?}");
    }

    Ok(())
}