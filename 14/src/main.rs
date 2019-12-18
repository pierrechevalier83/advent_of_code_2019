use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::ops::Mul;
use std::str::FromStr;

type ChemicalId = String;

#[derive(Clone)]
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
            Err("Incorrect reaction format".to_string())
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
    reactions: HashMap<ChemicalId, Reaction>,
}

impl FromStr for Nanofactory {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reactions = s
            .split('\n')
            .map(|r| {
                let reaction = Reaction::from_str(r)?;
                Ok((reaction.product.id.clone(), reaction))
            })
            .collect::<Result<_, Self::Err>>()?;
        Ok(Self { reactions })
    }
}

impl Debug for Nanofactory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for reaction in self.reactions.values() {
            writeln!(f, "{:?}", reaction)?;
        }
        Ok(())
    }
}

impl Nanofactory {
    fn from_ore(&self, id: &str) -> bool {
        let reaction = self.reactions[id].clone();
        reaction
            .reactants
            .iter()
            .all(|reactant| reactant.id == "ORE")
    }
    fn num_raw_needed_for(&self, id: &str) -> Vec<Chemical> {
        if self.from_ore(id) {
            vec![Chemical {
                quantity: 1,
                id: id.to_string(),
            }]
        } else {
            let reaction = self.reactions[id].clone();
            let n_product = reaction.product.quantity;
            reaction
                .reactants
                .iter()
                .flat_map(|reactant| {
                    self.num_raw_needed_for(&reactant.id)
                        .into_iter()
                        .map(move |c| c * reactant.quantity * n_product)
                })
                .collect()
        }
    }
    fn num_ore_needed_for(&self, id: &str, needed: usize) -> usize {
        let reaction = self.reactions[id].clone();
        let product = reaction.product.quantity;
        assert!(reaction.reactants.len() == 1);
        assert!(reaction.reactants[0].id == "ORE".to_string());
        let reactant = reaction.reactants[0].quantity;
        let ore = reactant
            * (needed / product + {
                if needed % product == 0 {
                    0
                } else {
                    1
                }
            });
        println!("for {} of {}, mine {} ore", needed, id, ore);
        ore
    }
    fn num_ore_needed_for_fuel(&self) -> usize {
        let chemical_id = |c: &Chemical| (*c).id.clone();
        let mut raws = self.num_raw_needed_for("FUEL");
        raws.sort_by(|left, right| chemical_id(left).cmp(&chemical_id(right)));
        println!("raws: {:?}", raws);
        raws.into_iter()
            .group_by(chemical_id)
            .into_iter()
            .map(|(id, group)| (id, group.into_iter().map(|c| c.quantity).sum()))
            .map(|(id, quantity)| self.num_ore_needed_for(&id, quantity))
            .sum()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_num_ore(input: &str, expected: usize) {
        let factory = Nanofactory::from_str(input).unwrap();
        let num_ore = factory.num_ore_needed_for_fuel();
        assert_eq!(expected, num_ore);
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
        );
        println!("\n\n");
        test_num_ore(
            "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
            165,
        );
    }
    #[test]
    fn test_larger_example() {
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
        );
    }
}
