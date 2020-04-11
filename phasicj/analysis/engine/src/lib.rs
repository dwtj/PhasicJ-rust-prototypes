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
    parse();
}

pub mod model;

pub struct Engine {
    id: String,

}

pub fn new(id: &str) -> Engine {
    Engine {
        id: String::from(id)
    }
}

pub fn mutate(e: &mut Engine) {
    e.id.push('h')
}
