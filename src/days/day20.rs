use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

use crate::etc::lib::lcm;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day20.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::U64(sol1(&parsed)), Solution::U64(sol2(&parsed)))
}

fn sol1(input: &Vec<String>) -> u64 {
    let modules = parse_modules(input);

    let pulses = find_pulses(&modules, 1000, false);

    pulses.0 * pulses.1
}

fn sol2(input: &Vec<String>) -> u64 {
    let modules = parse_modules(input);

    let pulses = find_pulses(&modules, 10_000, true);

    pulses.2
}

///////////////////////////////////////////////////////////////////////////////

fn parse_modules(input: &Vec<String>) -> Vec<(char, String, Vec<String>, Vec<String>)> {
    let mut res = vec![];

    for line in input {
        let parts: Vec<&str> = line.split(" -> ").collect();

        let module_type = parts[0].chars().next().unwrap();
        let name = parts[0].replace("%", "").replace("&", "");
        let output: Vec<String> = parts[1].split(", ").map(|s| s.to_string()).collect();

        res.push((module_type, name, vec![], output));
    }

    let resclone = res.clone();
    for module in res.iter_mut() {
        let mut inputs = vec![];
        for other in resclone.iter() {
            if other.3.contains(&module.1) {
                inputs.push(other.1.clone());
            }
        }
        module.2 = inputs;
    }

    res
}

fn find_pulses(
    input: &Vec<(char, String, Vec<String>, Vec<String>)>,
    push: u64,
    calc_rx: bool,
) -> (u64, u64, u64) {
    let mut flip_states: HashMap<String, bool> = HashMap::new();
    let mut conj_states: HashMap<String, HashMap<String, bool>> = HashMap::new();
    let mut pulses = [0; 2];
    let mut cur_pulses = VecDeque::new();

    let rxtarget = input
        .iter()
        .find(|m| m.3.contains(&"rx".to_string()))
        .unwrap();
    let mut rxinputs: HashMap<String, u64> =
        HashMap::from_iter(rxtarget.2.iter().map(|s| (s.clone(), 0)));

    for i in 0..push {
        cur_pulses.push_back(("button".to_string(), false, "broadcaster".to_string()));

        if calc_rx && rxinputs.iter().all(|(_, v)| *v > 0) {
            return (0, 0, rxinputs.iter().fold(1, |acc, (_, v)| lcm(acc, *v)));
        }

        while let Some(pulse) = cur_pulses.pop_front() {
            pulses[pulse.1 as usize] += 1;

            // If we're sending a pulse to the rx target, store the cycle count
            if pulse.2 == rxtarget.1 && pulse.1 == true {
                rxinputs.insert(pulse.0.clone(), i + 1);
            }

            let maybe_cur_module = &input.iter().find(|m| m.1 == pulse.2);
            if maybe_cur_module.is_none() {
                continue;
            }
            let cur_module = maybe_cur_module.unwrap();

            if cur_module.0 == '%' && pulse.1 == false {
                let cur_state = *flip_states
                    .entry(cur_module.1.clone())
                    .and_modify(|s| match s {
                        true => *s = false,
                        false => *s = true,
                    })
                    .or_insert(false);

                for other in cur_module.3.iter() {
                    cur_pulses.push_back((cur_module.1.to_string(), !cur_state, other.to_string()));
                }
            }

            if cur_module.0 == '&' {
                let cur_state = conj_states.entry(cur_module.1.clone()).or_insert_with(|| {
                    HashMap::from_iter(cur_module.2.iter().map(|s| (s.clone(), false)))
                });

                let cur_other_state = cur_state.entry(pulse.0.clone()).or_insert(false);
                *cur_other_state = pulse.1.clone();

                let new_pulse = !cur_state.iter().all(|(_, v)| *v == true);
                for other in cur_module.3.iter() {
                    cur_pulses.push_back((cur_module.1.to_string(), new_pulse, other.clone()));
                }
            }

            if cur_module.1 == "broadcaster" {
                for other in cur_module.3.iter() {
                    cur_pulses.push_back((cur_module.1.to_string(), pulse.1, other.clone()));
                }
            }
        }
    }

    (pulses[0], pulses[1], 0)
}

#[cfg(test)]
mod tests {
    use crate::day20;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day20::solve();

        assert_eq!(result.0, Solution::U64(896998430));
        assert_eq!(result.1, Solution::U64(236095992539963));
    }
}
