use anyhow::anyhow;
use pest::Parser;
use pest_parser::{Grammar, Rule};

#[test]
fn field_test() -> anyhow::Result<()> {
    let actual = Grammar::parse(Rule::field, "-1701.2006")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(actual.as_str(), "-1701.2006");
    assert_eq!(actual.as_span().start(), 0);
    assert_eq!(actual.as_span().end(), 10);

    let actual = Grammar::parse(Rule::field, "ERROR");
    assert!(actual.is_err());

    let actual = Grammar::parse(Rule::field, "");
    assert!(actual.is_err());

    Ok(())
}

#[test]
fn record_test() -> anyhow::Result<()> {
    let actual = Grammar::parse(Rule::record, "-1701.2006,19")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(actual.as_str(), "-1701.2006,19");
    assert_eq!(actual.as_span().start(), 0);
    assert_eq!(actual.as_span().end(), 13);

    let actual = Grammar::parse(Rule::record, "ERROR");
    assert!(actual.is_err());

    Ok(())
}

#[test]
fn file_test_single_record() -> anyhow::Result<()> {
    let actual = Grammar::parse(Rule::file, "-273.45,12\n")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;
    assert_eq!(actual.as_str(), "-273.45,12\n");

    Ok(())
}

#[test]
fn file_test_multiple_records() -> anyhow::Result<()> {
    let actual = Grammar::parse(Rule::file, "-273.45,12\n-273.45,12\n")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;
    assert_eq!(actual.as_str(), "-273.45,12\n-273.45,12\n");

    Ok(())
}

#[test]
fn file_test_invalid_records() -> anyhow::Result<()> {
    let actual = Grammar::parse(Rule::file, "-273.45 12");
    assert!(actual.is_err());

    let actual = Grammar::parse(Rule::file, "-273.45,12");
    assert!(actual.is_err());

    Ok(())
}
