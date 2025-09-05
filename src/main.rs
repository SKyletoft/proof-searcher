#![allow(incomplete_features, dead_code)]
#![feature(deref_patterns)]

mod prop;
use std::{
	collections::{HashSet, VecDeque},
	fmt,
	rc::Rc,
};

use prop::{
	Proposition::{self, *},
	and, or, var,
};

type Propositions = HashSet<Rc<Proposition>>;

fn main() {
	// Target:
	// ¬s → ¬r, (p ∧ q) ∨ r, ¬s → ¬q |- ¬p ∨ s

	let set1 = [
		Implies {
			left: Not(var('s')).into(),
			right: Not(var('r')).into(),
		},
		or(and(var('p'), var('q')).into(), var('r')),
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

	proof_search(set1, target)

	// let mut starting_node = SearchNode {
	//	premises: [
	//		Implies {
	//			left: Not(var('s')).into(),
	//			right: Not(var('r')).into(),
	//		},
	//		or(and(var('p'), var('q')).into(), var('r')),
	//		Implies {
	//			left: Not(var('s')).into(),
	//			right: Not(var('q')).into(),
	//		},
	//		Not(var('q')),
	//	]
	//	.into_iter()
	//	.map(Rc::new)
	//	.collect::<HashSet<_>>(),
	//	assumptions: vec![Hypothesis {
	//		assumption: Rc::new(Not(var('s'))),
	//		conclusions: [Rc::new(Not(var('s')))].into_iter().collect(),
	//	}],
	// };
	// eprintln!("{starting_node}");
	// join(&mut starting_node);
	// eprintln!("{starting_node}");
	// deduce(&mut starting_node);
	// eprintln!("{starting_node}");
	// let (a, vs) = conclusion_candidates(&starting_node).unwrap();
	// eprintln!("Conclusions:");
	// for v in vs.into_iter() {
	//	eprintln!(
	//		"\t{}",
	//		Implies {
	//			left: a.clone(),
	//			right: v
	//		}
	//	);
	// }
}

#[derive(Debug, Clone)]
struct Hypothesis {
	assumption: Rc<Proposition>,
	conclusions: Propositions,
}

impl Hypothesis {
	fn from_assumption(assumption: Rc<Proposition>) -> Self {
		let mut conclusions = HashSet::new();
		conclusions.insert(assumption.clone());
		Hypothesis {
			assumption,
			conclusions,
		}
	}
}

#[derive(Debug, Clone)]
struct SearchNode {
	premises: Propositions,
	assumptions: Vec<Hypothesis>,
}

impl fmt::Display for SearchNode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "{{\n\tpremises:")?;
		for premise in self.premises.iter() {
			writeln!(f, "\t\t{premise}")?;
		}
		writeln!(f, "\tassumptions:")?;
		for Hypothesis {
			assumption,
			conclusions,
		} in self.assumptions.iter()
		{
			writeln!(f, "\t\t{assumption}:")?;
			for new_conclusion in conclusions.iter() {
				writeln!(f, "\t\t\t{new_conclusion}")?;
			}
		}
		writeln!(f, "}}")?;

		Ok(())
	}
}

impl SearchNode {
	fn last(&self) -> &Propositions {
		self.assumptions
			.last()
			.map(|Hypothesis { conclusions, .. }| conclusions)
			.unwrap_or(&self.premises)
	}

	fn last_mut(&mut self) -> &mut Propositions {
		self.assumptions
			.last_mut()
			.map(|Hypothesis { conclusions, .. }| conclusions)
			.unwrap_or(&mut self.premises)
	}

	fn contains_except_last(&self, prop: &Proposition) -> bool {
		if self.assumptions.is_empty() {
			return false;
		}
		!self.premises.contains(prop)
			&& !self.assumptions[..self.assumptions.len() - 1]
				.iter()
				.any(|Hypothesis { conclusions, .. }| conclusions.contains(prop))
	}

	fn contains(&self, prop: &Proposition) -> bool {
		!self.premises.contains(prop)
			&& !self
				.assumptions
				.iter()
				.any(|Hypothesis { conclusions, .. }| conclusions.contains(prop))
	}
}

// We're doing a BFS-ish. Each node in the graph is a set of propositions and when we visit it we
// also deduce everything we can. Each edge is an assumption or a conclusion from an assumption.
// We visit assumptions in a simplest first order and conclusions before new assumptions.
fn proof_search(premises: Propositions, target: Proposition) {
	let mut queue = VecDeque::new();

	let mut starting_node = SearchNode {
		premises,
		assumptions: Vec::new(),
	};
	deduce(&mut starting_node);
	let cands = assumption_candidates(&starting_node.premises);
	for cand in cands.into_iter() {
		let premises = starting_node.premises.clone();
		queue.push_back(SearchNode {
			premises,
			assumptions: vec![Hypothesis::from_assumption(cand)],
		});
	}

	while let Some(mut node) = queue.pop_front() {
		eprintln!("{node}");

		if node.assumptions.len() >= 3 {
			continue;
		}
		if node.premises.contains(&target) {
			println!("{node}");
			break;
		}

		join(&mut node);
		deduce(&mut node);

		if let Some((assumption, c_cands)) = conclusion_candidates(&node) {
			let mut new = node.clone();
			new.assumptions.pop();
			for cand in c_cands.into_iter() {
				let mut node = new.clone();
				let props = node
					.assumptions
					.last_mut()
					.map(|Hypothesis { conclusions, .. }| conclusions)
					.unwrap_or(&mut node.premises);
				props.insert(Rc::new(Implies {
					left: assumption.clone(),
					right: cand,
				}));
			}
		}

		let last = node.last();
		let a_cands = assumption_candidates(last);
		for cand in a_cands.into_iter() {
			let premises = node.premises.clone();
			let mut assumptions = node.assumptions.clone();
			assumptions.push(Hypothesis::from_assumption(cand));
			queue.push_back(SearchNode {
				premises,
				assumptions,
			});
		}
	}
}

fn join(node: &mut SearchNode) -> &mut Propositions {
	if node.assumptions.is_empty() {
		return &mut node.premises;
	}
	let mid = node.assumptions.len() - 1;
	let (
		generations,
		[
			Hypothesis {
				assumption: new_assumption,
				conclusions: last,
			},
		],
	) = node.assumptions.split_at_mut(mid)
	else {
		unreachable!()
	};
	for known in node.premises.iter().chain(
		generations
			.iter()
			.flat_map(|Hypothesis { conclusions, .. }| conclusions.iter()),
	) {
		last.insert(Rc::new(and(new_assumption.clone(), known.clone())));
	}
	last
}

fn deduce(node: &mut SearchNode) {
	let mut set1;
	let mut set2 = node.last().clone();

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

	*node.last_mut() = set2
		.into_iter()
		.filter(|p| !node.contains_except_last(p))
		.collect();
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

fn conclusion_candidates(node: &SearchNode) -> Option<(Rc<Proposition>, Vec<Rc<Proposition>>)> {
	let Hypothesis {
		assumption,
		conclusions,
	} = node.assumptions.last()?;

	let mut v = conclusions
		.iter()
		.filter(|p| node.contains(p) && p != &assumption && !p.is_anded_with(assumption))
		.cloned()
		.collect::<Vec<Rc<Proposition>>>();
	v.sort_unstable_by_key(|p| p.len());

	Some((assumption.clone(), v))
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
		out.insert(Rc::new(and(x.clone(), Rc::new(and(y.clone(), z.clone())))));
	}

	if let And {
		left: x,
		right: And { left: y, right: z },
	} = prop
	{
		out.insert(Rc::new(and(Rc::new(and(x.clone(), y.clone())), z.clone())));
	}

	if let Or {
		left: Or { left: x, right: y },
		right: z,
	} = prop
	{
		out.insert(Rc::new(or(x.clone(), Rc::new(or(y.clone(), z.clone())))));
	}

	if let Or {
		left: x,
		right: Or { left: y, right: z },
	} = prop
	{
		out.insert(Rc::new(or(Rc::new(or(x.clone(), y.clone())), z.clone())));
	}

	// And-elimination + And-reordering
	if let And { left, right } = prop {
		out.insert(left.clone());
		out.insert(right.clone());
	}

	// Or-reordering
	if let Or { left, right } = prop {
		out.insert(Rc::new(or(right.clone(), left.clone())));
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
	if let And {
		left: Implies { left: y, right: z },
		right: x,
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
	if let And {
		left: Not(z),
		right: Implies { left: x, right: y },
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
