use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result<()>{
    let a = Grammar::parse(Rule::file, "-1701.2006,-19,25\n")?;
    println!("{:?}", a);

    Ok(())
}