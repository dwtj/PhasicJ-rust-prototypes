use phasicj_analysis_language::parse;

// While the engine is running, it is responsible for:
//
// - listening for and handing out handles with which in-process concurrent
// components can interact with the engine.
// - relaying new facts along handles which subscribed to that kind of fact.
// - Infering *relevant* new facts from old via axioms.
// - Storing facts.
// - Storing events.
// - Reporting conflicts: when analyses report both p & !p.
pub fn hello() {
    assert!(parse("f(X).").is_ok());
}
