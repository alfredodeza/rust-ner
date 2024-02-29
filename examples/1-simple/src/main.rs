extern crate anyhow;

use rust_bert::pipelines::ner::NERModel;

fn main() -> anyhow::Result<()> {
    //    Set-up model
    let ner_model = NERModel::new(Default::default())?;

    //    Define input
    let input = [
        "My name is Alfredo and I was born in Peru.",
        "In Portugal the pastel de nata is great.",
        "I used to work in a small company called ShootQ",
    ];

    //    Run model
    let output = ner_model.predict_full_entities(&input);
    for entity in output {
        println!("{entity:?}");
    }

    Ok(())
}