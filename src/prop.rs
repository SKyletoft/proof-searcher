use std::{fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Proposition {
	Variable(u8),
	And {
		left: Rc<Proposition>,
		right: Rc<Proposition>,
	},
	Or {
		left: Rc<Proposition>,
		right: Rc<Proposition>,
	},
	Implies {
		left: Rc<Proposition>,
		right: Rc<Proposition>,
	},
	Not(Rc<Proposition>),
}

impl fmt::Display for Proposition {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fn maybe_wrapped(p: &Proposition, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			match p {
				x @ (Proposition::Variable(_) | Proposition::Not(Proposition::Variable(_))) => {
					write!(f, "{x}")
				}
				_ => write!(f, "({p})"),
			}
		}

		match self {
			Proposition::Variable(v) => write!(f, "{}", (*v + b'a') as char),

			Proposition::Not(inner) => {
				write!(f, "¬")?;
				maybe_wrapped(inner, f)
			}

			Proposition::And { left, right } => {
				maybe_wrapped(left, f)?;
				write!(f, " ∧ ")?;
				maybe_wrapped(right, f)
			}

			Proposition::Or { left, right } => {
				maybe_wrapped(left, f)?;
				write!(f, " ∨ ")?;
				maybe_wrapped(right, f)
			}

			Proposition::Implies { left, right } => {
				maybe_wrapped(left, f)?;
				write!(f, " → ")?;
				maybe_wrapped(right, f)
			}
		}
	}
}
