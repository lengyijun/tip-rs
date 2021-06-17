use pest::Parser;

#[derive(Parser)]
#[grammar = "tip.pest"]
struct IdentParser;

pub fn parse(input: &str) {
    let pairs = IdentParser::parse(Rule::program, input).unwrap_or_else(|e| panic!("{}", e));
}

#[cfg(test)]
mod tests {
    use crate::ast_parser::parse;
    use std::fs;

    #[test]
    fn test_parse() -> std::io::Result<()> {
        for entry in fs::read_dir("/home/lyj/TIP/examples")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let content = &fs::read_to_string(&path)?;
                dbg!("{:?}", &path);
                parse(&content);
            }
        }
        Ok(())
    }
}
