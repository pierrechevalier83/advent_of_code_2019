use itertools::Itertools;
use petgraph::{
    dot::Dot,
    graph::{DiGraph, NodeIndex},
    visit::{Reversed, Topo},
    Direction,
};
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::ops::{Div, Mul};
use std::str::FromStr;

type ChemicalId = String;

#[derive(Clone, Default)]
struct Chemical {
    id: ChemicalId,
    quantity: usize,
}

impl FromStr for Chemical {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((quantity, id)) = s.trim().split(' ').collect_tuple() {
            Ok(Self {
                id: id.to_string(),
                quantity: quantity.parse().map_err(|e| format!("{}", e))?,
            })
        } else {
            Err("Incorrect chemical format".to_string())
        }
    }
}

impl Debug for Chemical {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.quantity, self.id)
    }
}

impl Mul<usize> for Chemical {
    type Output = Self;

    fn mul(self, quantity: usize) -> Self {
        Self {
            id: self.id,
            quantity: self.quantity * quantity,
        }
    }
}

impl Div<usize> for Chemical {
    type Output = Self;

    fn div(self, quantity: usize) -> Self {
        Self {
            id: self.id,
            quantity: self.quantity / quantity,
        }
    }
}

#[derive(Clone)]
struct Reaction {
    reactants: Vec<Chemical>,
    product: Chemical,
}

impl FromStr for Reaction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((reactants, product)) = s.trim().split(" => ").collect_tuple() {
            let reactants = reactants
                .split(", ")
                .map(Chemical::from_str)
                .collect::<Result<Vec<_>, _>>()?;
            let product = Chemical::from_str(product)?;
            Ok(Self { reactants, product })
        } else {
            Err(format!("Incorrect reaction format: '{}'", s))
        }
    }
}

impl Debug for Reaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (index, reactant) in self.reactants.iter().enumerate() {
            write!(f, "{:?}", reactant)?;
            if index != self.reactants.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, " => {:?}", self.product)
    }
}

struct Nanofactory {
    graph: DiGraph<Chemical, usize>,
}

impl FromStr for Nanofactory {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reactions = s
            .trim()
            .split('\n')
            .map(Reaction::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let nodes = std::iter::once(Chemical {
            id: "ORE".to_string(),
            quantity: 1,
        })
        .chain(reactions.iter().map(|reaction| reaction.product.clone()))
        .collect::<Vec<_>>();
        let as_index = |id: &'_ String| nodes.iter().position(|x| id == &x.id).unwrap() as u32;
        let edges = reactions
            .iter()
            .flat_map(|reaction| {
                reaction
                    .reactants
                    .iter()
                    .map(|reactant| {
                        (
                            as_index(&reactant.id),
                            as_index(&reaction.product.id),
                            reactant.quantity,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut graph = DiGraph::from_edges(edges);
        for (vertex, name) in graph.node_weights_mut().zip(nodes.iter()) {
            *vertex = name.clone();
        }
        Ok(Self { graph })
    }
}

impl Debug for Nanofactory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", Dot::new(&self.graph))
    }
}

impl Nanofactory {
    fn num_ore_needed_for_fuel(&self, n_needed: usize) -> usize {
        // For each chemical, how many times must I run the reaction which produces it
        let mut product_needed = HashMap::new();
        let mut topo = Topo::new(Reversed(&self.graph));
        let fuel = topo.next(Reversed(&self.graph)).unwrap();
        product_needed.insert(
            fuel,
            Self::divide_and_round_up(n_needed, self.product(fuel).quantity),
        );
        while let Some(reaction_index) = topo.next(Reversed(&self.graph)) {
            product_needed.insert(
                reaction_index,
                self.calculate_n_reactions(&product_needed, reaction_index),
            );
        }
        let ore = &self.graph.externals(Direction::Incoming).next().unwrap();
        product_needed[ore]
    }
    fn product(&self, reaction_index: NodeIndex) -> Chemical {
        self.graph.node_weight(reaction_index).unwrap().clone()
    }
    /// Number of times we need to run the reaction which produces this chemical
    fn calculate_n_reactions(
        &self,
        product_needed: &HashMap<NodeIndex, usize>,
        reaction_index: NodeIndex,
    ) -> usize {
        let product = self.product(reaction_index);
        Self::divide_and_round_up(
            // These are the nodes which consume the product of this reaction.
            // They determine how much quantity of product is needed
            self.graph
                .neighbors_directed(reaction_index, Direction::Outgoing)
                .into_iter()
                .map(|consumer| {
                    self.graph
                        .edge_weight(self.graph.find_edge(reaction_index, consumer).unwrap())
                        .unwrap()
                        * product_needed[&consumer]
                })
                .sum(),
            product.quantity,
        )
    }
    fn divide_and_round_up(x: usize, y: usize) -> usize {
        x / y + {
            if x % y == 0 {
                0
            } else {
                1
            }
        }
    }
    fn num_fuel_produced_by_one_trillion_ore(&self) -> usize {
        let one_trillion = 1_000_000_000_000;
        let mut guess = Self::divide_and_round_up(one_trillion, self.num_ore_needed_for_fuel(1));
        let mut low_guess = 0;
        let mut high_guess = one_trillion;
        while high_guess > low_guess + 1 {
            let result = self.num_ore_needed_for_fuel(guess);
            if result > one_trillion {
                high_guess = guess;
            } else if result < one_trillion {
                low_guess = guess;
            } else {
                return guess;
            }
            guess = (high_guess + low_guess) / 2;
            //    println!("({} - {}), {} -> {}", low_guess, high_guess, guess, result);
        }
        guess
    }
}

fn main() {
    let factory = Nanofactory::from_str(include_str!("input.txt")).unwrap();
    let part_1 = factory.num_ore_needed_for_fuel(1);
    assert_eq!(378929, part_1);
    println!("part 1: {}", part_1);
    let part_2 = factory.num_fuel_produced_by_one_trillion_ore();
    println!("part 2: {}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_num_ore(input: &str, expected: usize, expected_trillion_ore: usize) {
        let factory = Nanofactory::from_str(input).unwrap();
        let num_ore = factory.num_ore_needed_for_fuel(1);
        assert_eq!(expected, num_ore);
        let num_fuel = factory.num_fuel_produced_by_one_trillion_ore();
        assert_eq!(expected_trillion_ore, num_fuel);
    }
    #[test]
    fn test_small_example() {
        test_num_ore(
            "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
            31,
            34482758620,
        );
        test_num_ore(
            "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
            165,
            6323777403,
        );
    }
    #[test]
    fn test_larger_example() {
        test_num_ore(
            "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
            13312,
            82892753,
        );
        test_num_ore(
            "171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX",
            2210736,
            460664,
        );
    }
}
