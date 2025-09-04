#![feature(deref_patterns)]

mod prop;
use std::{
	collections::{HashSet, VecDeque},
	fmt,
	rc::Rc,
};

use prop::Proposition::{self, *};

type Propositions = HashSet<Rc<Proposition>>;

fn main() {
	// Target:
	// ¬s → ¬r, (p ∧ q) ∨ r, ¬s → ¬q |- ¬p ∨ s

	let mut set1 = [
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
		(*var('s')).clone(),
	]
	.into_iter()
	.map(Rc::new)
	.collect::<HashSet<_>>();

	let target = Or {
		left: Rc::new(Not(var('p'))),
		right: var('s'),
	};

	set1 = deduce(set1);

#[derive(Clone)]
struct SearchNode {
	premises: Propositions,
	assumptions: Vec<(Rc<Proposition>, Propositions)>,
}

	let cands = assumption_candidates(&set1);

	println!("{}", set1.len());
	println!("{}", set1.contains(&target));
	for prop in set1.into_iter() {
		println!("{prop}");
	}
	println!("-----------------");
	for prop in cands.into_iter() {
		println!("{prop}");
	}
}

// We're doing a BFS-ish. Each node in the graph is a set of propositions and when we visit it we
// also deduce everything we can. Each edge is an assumption or a conclusion from an assumption.
// We visit assumptions in a simplest first order and conclusions before new assumptions.
fn proof_search(premises: Propositions, target: Proposition) {

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

fn assumption_candidates(facts: &HashSet<Rc<Proposition>>) -> Vec<Rc<Proposition>> {
	let mut out = HashSet::new();

	fn recurse(
		cand: Rc<Proposition>,
		facts: &HashSet<Rc<Proposition>>,
		out: &mut HashSet<Rc<Proposition>>,
	) {
		if !facts.contains(&cand) {
			out.insert(cand.clone());
		}
		match &cand {
			And { left, right } | Or { left, right } | Implies { left, right } => {
				recurse(left.clone(), facts, out);
				recurse(right.clone(), facts, out);
			}
			Not(inner) => {
				recurse(inner.clone(), facts, out);
			}
			_ => {}
		}
	}

	for fact in facts.iter().cloned() {
		recurse(fact, facts, &mut out);
	}

	let mut v = out.into_iter().collect::<Vec<Rc<Proposition>>>();
	v.sort_unstable_by_key(|p| p.len());
	v
}

// Just check against every known rule and collect all conclusions.
fn single_prop_conclusions(prop: &Proposition) -> HashSet<Rc<Proposition>> {
	let mut out = HashSet::new();

	// Rebalancing
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

	if let And {
		left: x,
		right: And { left: y, right: z },
	} = prop
	{
		out.insert(Rc::new(And {
			left: Rc::new(And {
				left: x.clone(),
				right: y.clone(),
			}),
			right: z.clone(),
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

	if let Or {
		left: x,
		right: Or { left: y, right: z },
	} = prop
	{
		out.insert(Rc::new(Or {
			left: Rc::new(Or {
				left: x.clone(),
				right: y.clone(),
			}),
			right: z.clone(),
		}));
	}

	// And-elimination + And-reordering
	if let And { left, right } = prop {
		out.insert(Rc::new(And {
			left: right.clone(),
			right: left.clone(),
		}));
		out.insert(left.clone());
		out.insert(right.clone());
	}

	// Or-reordering
	if let Or { left, right } = prop {
		out.insert(Rc::new(Or {
			left: right.clone(),
			right: left.clone(),
		}));
	}

	// Implication-elimination
	if let And {
		left: x,
		right: Implies { left: y, right: z },
	} = prop
		&& x == y
	{
		out.insert(z.clone());
	}

	// Double negation elimination
	if let Not(Not(x)) = prop {
		out.insert(x.clone());
	}

	// if !matches!(prop, Not(_)) {
	//	out.insert(Rc::new(Not(Rc::new(Not(Rc::new(prop.clone()))))));
	// }

	// Proof by contradiction
	if let Implies {
		left,
		right: Bottom,
	} = prop
	{
		out.insert(Rc::new(Not(left.clone())));
	}

	// MT
	if let And {
		left: Implies { left: x, right: y },
		right: Not(z),
	} = prop
		&& z == y
	{
		out.insert(Rc::new(Not(x.clone())));
	}

	// Not-elimination
	if let And {
		left,
		right: Not(x),
	} = prop
		&& left == x
	{
		out.insert(Rc::new(Bottom));
	}

	out
}
