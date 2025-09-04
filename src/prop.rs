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
	Bottom,
}

impl Proposition {
	pub fn len(&self) -> usize {
		match self {
			Proposition::Bottom => 0,
			Proposition::Variable(_) => 1,
			Proposition::Not(Proposition::Not(proposition)) => 3 + proposition.len(),
			Proposition::Not(proposition) => 1 + proposition.len(),
			Proposition::And { left, right }
			| Proposition::Or { left, right }
			| Proposition::Implies { left, right } => 1 + left.len() + right.len(),
		}
	}
}

impl fmt::Display for Proposition {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fn maybe_wrapped(p: &Proposition, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			match p {
				x @ (Proposition::Bottom
				| Proposition::Variable(_)
				| Proposition::Not(Proposition::Not(Proposition::Variable(_)))
				| Proposition::Not(Proposition::Variable(_))) => {
					write!(f, "{x}")
				}
				_ => write!(f, "({p})"),
			}
		}

		match self {
			Proposition::Bottom => write!(f, "⊥"),

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
