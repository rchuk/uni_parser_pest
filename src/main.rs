use anyhow::Result;
use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> Result<()> {
    let field = Grammar::parse(Rule::field, "-273.15")?;
    println!("{:#?}", field);
    let record = Grammar::parse(Rule::record, "-273.15,-5,4,3.14")?;
    println!("{:#?}", record);

    Ok(())
}