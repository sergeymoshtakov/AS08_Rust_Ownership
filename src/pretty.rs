use crate::term::*;
use std::fmt;

/// Pretty prints a term.
pub fn pretty_print(term: &Term) -> String {
    match term {
        Term::Var(x) => x.clone(),
        Term::Abs(param, body) => {
            format!("λ{}. {}", param, pretty_print(body))
        }
        Term::App(t1, t2) => {
            let s1 = match **t1 {
                Term::Abs(_, _) => format!("({})", pretty_print(t1)),
                _ => pretty_print(t1),
            };
            let s2 = match **t2 {
                Term::Var(_) => pretty_print(t2),
                _ => format!("({})", pretty_print(t2)),
            };
            format!("{} {}", s1, s2)
        }
    }
}

/// Display trait implementation for Term.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self))
    }
}