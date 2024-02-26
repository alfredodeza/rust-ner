extern crate anyhow;

use rust_bert::pipelines::ner::NERModel;

fn main() -> anyhow::Result<()> {
    //    Set-up model
    let ner_model = NERModel::new(Default::default())?;

    //    Define input
    let input = [
        "My name is Amélie. I live in Москва.",
        "Chongqing is a city in China.",
        "Asked John Smith about Acme Corp",
    ];

    //    Run model
    let output = ner_model.predict_full_entities(&input);
    for entity in output {
        println!("{entity:?}");
    }

    Ok(())
}