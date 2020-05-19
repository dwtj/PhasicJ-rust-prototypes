extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

type PjParseResult<'a> = Result<::pest::iterators::Pairs<'a, Rule>, ::pest::error::Error<Rule>>;

pub fn parse<'a>(program: &'a str) -> PjParseResult<'a> {
    parse_with_rule(program, Rule::program)
}

pub fn parse_with_rule<'a>(str: &'a str, rule: Rule) -> PjParseResult<'a> {
    PjAnalysisLanguageParser::parse(rule, str)
}

#[derive(Parser)]
#[grammar = "pjal.pest"]
pub struct PjAnalysisLanguageParser;

#[cfg(test)]
// TODO(dwtj): Consider using pest-parser's `parses_to` and `fails_with` test
//  helper functions.
mod tests {
    use crate::PjAnalysisLanguageParser;
    use pest::Parser;
    use crate::Rule;
    use pest::parses_to;
    use pest::fails_with;
    use pest::consumes_to;

    #[test]
    fn smoke_test_parse_function_from_parser_impl() {
        let prog = "r(x, X).";
        let res = PjAnalysisLanguageParser::parse(Rule::program, prog);
        assert!(res.is_ok());
    }

    #[test]
    fn smoke_test_parse_function_from_crate_root() {
        let prog = "r(x, X).";
        let res = crate::parse(prog);
        assert!(res.is_ok());
    }

    #[test]
    fn smoke_test_parse_with_rule_function_from_crate_root() {
        let prog = "r(x, X).";
        let res = crate::parse_with_rule(prog, Rule::program);
        assert!(res.is_ok());
    }

    #[test]
    fn parse_program() {
        parses_to!(
            parser: PjAnalysisLanguageParser,
            input:  "p(x, X).",
            rule:   Rule::program,
            tokens: [
                program(0, 8, [
                    axiom(0, 8, [
                        term(0, 7, [
                            relation(0, 1, []),
                            atom(2, 3, []),
                            variable(5, 6, []),
                        ]),
                    ]),
                ])
            ]
        );
    }

    #[test]
    fn parse_rule_statement() {
        parses_to!(
            parser: PjAnalysisLanguageParser,
            input:  "r(x) <- r(y).",
            rule:   Rule::statement,
            tokens: [
                statement(0, 13, [
                    rule(0, 13, [
                        rule_head(0, 4, [
                            term(0, 4, [
                                relation(0, 1, []),
                                atom(2, 3, []),
                            ])
                        ]),
                        rule_body(8, 12, [
                            term(8, 12, [
                                relation(8, 9, []),
                                atom(10, 11),
                            ])
                        ])
                    ])
                ])
            ]
        );
    }

    #[test]
    fn parse_query() {
        parses_to!(
            parser: PjAnalysisLanguageParser,
            input:  "p(X)?",
            rule:   Rule::query,
            tokens: [
                query(0, 5, [
                    term(0, 4, [
                        relation(0, 1, []),
                        variable(2, 3, [])
                    ])
                ])
            ]
        );
    }

    #[test]
    fn parse_atom() {
        parses_to!(
            parser: PjAnalysisLanguageParser,
            input:  "a",
            rule:   Rule::atom,
            tokens: [
                atom(0, 1, [])
            ]
        );
    }

    #[test]
    fn bad_atom_parse_should_fail() {
        fails_with!(
            parser:    PjAnalysisLanguageParser,
            input:     "X",
            rule:      Rule::atom,
            positives: vec![Rule::atom],
            negatives: vec![],
            pos:       0
        );
    }

    #[test]
    fn bad_statement_parse_should_fail() {
        let input = "This is not a statement.";
        fails_with!(
            parser: PjAnalysisLanguageParser,
            input: input,
            rule: Rule::statement,
            positives: vec![Rule::statement],
            negatives: vec![],
            pos: 0
        );
    }
}