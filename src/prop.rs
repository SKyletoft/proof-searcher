use std::fmt;
use std::rc::Rc;

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

impl Proposition {
	fn precedence(&self) -> i8 {
		match self {
			Proposition::Variable(_) => 4,
			Proposition::Not(_) => 3,
			Proposition::And { .. } => 2,
			Proposition::Or { .. } => 1,
			Proposition::Implies { .. } => 0,
		}
	}
}

impl fmt::Display for Proposition {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fn fmt_with_prec(
			p: &Proposition,
			parent_prec: i8,
			f: &mut fmt::Formatter<'_>,
		) -> fmt::Result {
			let prec = p.precedence();
			let needs_parens = prec < parent_prec;

			if needs_parens {
				write!(f, "(")?;
			}

			match p {
				Proposition::Variable(v) => write!(f, "{}", (*v + b'a') as char)?,

				Proposition::Not(inner) => {
					write!(f, "¬")?;
					fmt_with_prec(inner, prec, f)?;
				}

				Proposition::And { left, right } => {
					fmt_with_prec(left, prec, f)?;
					write!(f, " ∧ ")?;
					fmt_with_prec(right, prec, f)?;
				}

				Proposition::Or { left, right } => {
					fmt_with_prec(left, prec, f)?;
					write!(f, " ∨ ")?;
					fmt_with_prec(right, prec, f)?;
				}

				Proposition::Implies { left, right } => {
					fmt_with_prec(left, prec, f)?;
					write!(f, " → ")?;
					// right-associative: make the right child bind *tighter*
					fmt_with_prec(right, prec - 1, f)?;
				}
			}

			if needs_parens {
				write!(f, ")")?;
			}
			Ok(())
		}

		fmt_with_prec(self, 0, f)
	}
}

