use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

fn is_well_formed_name(n: &dyn Name) -> bool {
    str_is_well_formed_name(n.name())
}

fn str_is_well_formed_name(s: &str) -> bool {
    // TODO(dwtj): Everything!
    panic!("TODO: Unimplemented method.");
}

pub trait Name : Display {
    fn name(&self) -> &str;

    fn name_eq(&self, other: &dyn Name) -> bool {
        self.name().eq(other.name())
    }

    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.name())
    }
}

pub trait RelationName: Name {}

pub trait Argument : Name {}

pub trait Atom : Argument {}

pub trait Variable : Argument {}

pub trait Term : Display {
    fn relation_name(&self) -> &dyn RelationName;
    fn arguments(&self) -> &Vec<Box<dyn Argument>>;

    fn fmt(&self, f: &mut Formatter) -> Result {
        //write!(f, "{}", self.name())
        panic!("TODO: Unimplemented method!")
    }
}

pub trait Axiom : Statement {
    fn head(&self) -> &dyn Term;

    fn fmt(&self, f: &mut Formatter) -> Result {
        //write!(f, "{}", self.name());
        // TODO(dwtj): Everything!
        panic!("TODO: Unimplemented method!");
    }
}

pub trait Rule : Statement {
    fn head(&self) -> &dyn Term;
    fn conjuncts(&self) -> &Vec<Box<dyn Term>>;

    fn fmt(&self, f: &mut Formatter) -> Result {
        // TODO(dwtj): Everything!
        panic!("TODO: Unimplemented method!");
    }
}

pub trait Query : Statement {
    fn conjuncts(&self) -> &Vec<Box<dyn Term>>;

    fn fmt(&self, f: &mut Formatter) -> Result {
        let iter = self.conjuncts().iter();
        match iter.next() {
            case None: return ""
        }

        //write!(f, "{} ", self.head(), self);
        // TODO(dwtj): Everything!
        panic!("TODO: Unimplemented method!");
    }
}

pub trait Statement : Display {}