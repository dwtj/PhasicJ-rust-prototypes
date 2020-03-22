extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "pjal.pest"]
struct PjAnalysisLanguageParser;

pub fn parse() {
    let successful_parse = PjAnalysisLanguageParser::parse(Rule::program, "p(x, x).");
    println!("{:?}", successful_parse);

    //let successful_parse = PjAnalysisLanguageParser::parse(Rule::statement, "X");
    //println!("{:?}", successful_parse);

    //let unsuccessful_parse = PjAnalysisLanguageParser::parse(Rule::statement, "this is not a statement");
    //println!("{:?}", unsuccessful_parse);
}