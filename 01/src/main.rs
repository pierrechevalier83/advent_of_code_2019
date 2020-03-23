#![deny(warnings)]

/// The Elves quickly load you into a spacecraft and prepare to launch.

/// At the first Go / No Go poll, every Elf is Go until the Fuel Counter-Upper. They haven't determined the amount of fuel required yet.

/// Fuel required to launch a given module is based on its mass. Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.

/// For example:

///    For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
///    For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
///    For a mass of 1969, the fuel required is 654.
///    For a mass of 100756, the fuel required is 33583.

/// The Fuel Counter-Upper needs to know the total fuel requirement. To find it, individually calculate the fuel needed for the mass of each module (your puzzle input), then add together all the fuel values.

/// What is the sum of the fuel requirements for all of the modules on your spacecraft?

mod naive {
    pub(super) fn fuel_required_to_launch_module(mass: u32) -> u32 {
        if mass / 3 >= 2 {
            mass / 3 - 2
        } else {
            0
        }
    }
}

mod correct {
    pub(super) fn fuel_required_to_launch_module(mass: u32) -> u32 {
        let naive = super::naive::fuel_required_to_launch_module(mass);
        if naive == 0 {
            0
        } else {
            let required_for_mass = naive;
            let required_for_fuel = fuel_required_to_launch_module(required_for_mass);
            required_for_mass + required_for_fuel
        }
    }
}

fn parse_input() -> Vec<u32> {
    let data = include_str!("input.txt");
    data.split("\n")
        .filter(|s| *s != "")
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let data = parse_input();
    let naive_result: u32 = data
        .clone()
        .into_iter()
        .map(naive::fuel_required_to_launch_module)
        .sum();
    assert_eq!(3315383, naive_result);
    println!("part 1: {}", naive_result);
    let correct_result: u32 = data
        .into_iter()
        .map(correct::fuel_required_to_launch_module)
        .sum();
    assert_eq!(4970206, correct_result);
    println!("part 2: {}", correct_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_naive_fuel_required_to_launch_module_when_all_goes_well() {
        assert_eq!(2, naive::fuel_required_to_launch_module(12));
        assert_eq!(2, naive::fuel_required_to_launch_module(14));
        assert_eq!(654, naive::fuel_required_to_launch_module(1969));
        assert_eq!(33583, naive::fuel_required_to_launch_module(100756));
    }
    #[test]
    fn test_naive_fuel_required_to_launch_module_when_mass_is_low() {
        assert_eq!(0, naive::fuel_required_to_launch_module(1));
        assert_eq!(0, naive::fuel_required_to_launch_module(5));
    }
    #[test]
    fn test_correct_fuel_required_to_launch_module_when_all_goes_well() {
        assert_eq!(2, correct::fuel_required_to_launch_module(12));
        assert_eq!(2, correct::fuel_required_to_launch_module(14));
        assert_eq!(966, correct::fuel_required_to_launch_module(1969));
        assert_eq!(50346, correct::fuel_required_to_launch_module(100756));
    }
}
