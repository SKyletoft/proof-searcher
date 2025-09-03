#![feature(deref_patterns)]

mod prop;
use std::{collections::HashSet, rc::Rc};

use prop::Proposition::{self, *};

fn main() {
	// Target:
	// ¬s → ¬r, (p ∧ q) ∨ r, ¬s → ¬q |- ¬p ∨ s

	let set1 = [
		Implies {
			left: Not(var('s')).into(),
			right: Not(var('r')).into(),
		},
		Or {
			left: And {
				left: var('p'),
				right: var('q'),
			}
			.into(),
			right: var('r'),
		},
		Implies {
			left: Not(var('s')).into(),
			right: Not(var('q')).into(),
		},
	]
	.into_iter()
	.map(Rc::new)
	.collect::<HashSet<_>>();

	let target = Or {
		left: Rc::new(Not(var('p'))),
		right: var('s'),
	};

	println!("{}", set1.len());
	println!("{}", set1.contains(&target));
	for prop in set1.into_iter() {
		println!("{prop}");
	}
}

fn var(c: char) -> Rc<Proposition> {
	Rc::new(Variable(c as u8 - b'a'))
}

fn deduce(facts: HashSet<Rc<Proposition>>) -> HashSet<Rc<Proposition>> {
	let mut set1;
	let mut set2 = facts;

	loop {
		let fact_count = set2.len();
		set1 = set2;
		set2 = HashSet::new();
		for prop in set1.into_iter() {
			for new_prop in single_prop_conclusions(&prop) {
				set2.insert(new_prop);
			}
			set2.insert(prop);
		}
		if set2.len() == fact_count {
			break;
		}
	}

	set2
}

// Just check against every known rule and collect all conclusions.
fn single_prop_conclusions(prop: &Proposition) -> HashSet<Rc<Proposition>> {
	let mut out = HashSet::new();

	if let And {
		left: And { left: x, right: y },
		right: z,
	} = prop
	{
		out.insert(Rc::new(And {
			left: x.clone(),
			right: Rc::new(And {
				left: y.clone(),
				right: z.clone(),
			}),
		}));
	}
	if let And { left, right } = prop {
		out.insert(Rc::new(And {
			left: right.clone(),
			right: left.clone(),
		}));
	}

	if let Or {
		left: Or { left: x, right: y },
		right: z,
	} = prop
	{
		out.insert(Rc::new(Or {
			left: x.clone(),
			right: Rc::new(Or {
				left: y.clone(),
				right: z.clone(),
			}),
		}));
	}
	if let Or { left, right } = prop {
		out.insert(Rc::new(Or {
			left: right.clone(),
			right: left.clone(),
		}));
	}

	if let And { left, right } = prop {
		out.insert(left.clone());
		out.insert(right.clone());
	}
	if let Not(Not(x)) = prop {
		out.insert(x.clone());
	}
	if let Not(_) = prop {
		todo!()
	}

	out
}
