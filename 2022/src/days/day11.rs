use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let sol1: u64 = sol1(&mut monkeys(), &mut items());
    let sol2: u64 = sol2(&mut monkeys(), &mut items(), 9699690);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn sol1(monkeys: &mut Vec<Monkey>, items: &mut Vec<Vec<Item>>) -> u64 {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            let cur_items = items[i].clone();
            items[i] = vec![];

            for level in cur_items {
                monkey.inspected += 1;
                let newlevel = (monkey.inspect)(level) / 3;
                let newmonkey = (monkey.test)(newlevel);
                items[newmonkey].push(newlevel);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));

    (monkeys[0].inspected * monkeys[1].inspected) as u64
}

fn sol2(monkeys: &mut Vec<Monkey>, items: &mut Vec<Vec<Item>>, common_multiple: i64) -> u64 {
    for j in 1..=10_000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let cur_items = items[i].clone();
            items[i] = vec![];

            for level in cur_items {
                let mut newlevel = monkey.inspect(level);

                while newlevel > common_multiple {
                    newlevel = newlevel - common_multiple;
                }

                let newmonkey = monkey.test(newlevel);
                items[newmonkey].push(newlevel);
            }
        }

        if j % 1_000 == 0 || j == 20 || j == 1 {
            // println!(
            //     "{:?}",
            //     (j, monkeys.iter().map(|m| m.inspected).collect::<Vec<i64>>())
            // );
        }
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));

    (monkeys[0].inspected * monkeys[1].inspected) as u64
}

///////////////////////////////////////////////////////////////////////////////

type Item = i64;

#[derive(Debug, Clone)]
struct Monkey {
    inspected: i64,
    inspect: fn(item: Item) -> Item,
    test: fn(item: Item) -> usize,
}

impl Monkey {
    fn new(inspect: fn(item: Item) -> Item, test: fn(item: Item) -> usize) -> Monkey {
        Monkey {
            inspected: 0,
            inspect: inspect,
            test: test,
        }
    }
    fn inspect(&mut self, item: Item) -> Item {
        self.inspected += 1;
        (self.inspect)(item)
    }
    fn test(&mut self, item: Item) -> usize {
        (self.test)(item)
    }
}

#[allow(unused)]
fn example_monkeys() -> Vec<Monkey> {
    vec![
        Monkey::new(
            |item: Item| item * 19,
            |item: Item| {
                if item % 23 == 0 {
                    2
                } else {
                    3
                }
            },
        ),
        Monkey::new(
            |item: Item| item + 6,
            |item: Item| {
                if item % 19 == 0 {
                    2
                } else {
                    0
                }
            },
        ),
        Monkey::new(
            |item: Item| item * item,
            |item: Item| {
                if item % 13 == 0 {
                    1
                } else {
                    3
                }
            },
        ),
        Monkey::new(
            |item: Item| item + 3,
            |item: Item| {
                if item % 17 == 0 {
                    0
                } else {
                    1
                }
            },
        ),
    ]
}

#[allow(unused)]
fn example_items() -> Vec<Vec<Item>> {
    vec![
        vec![79, 98],
        vec![54, 65, 75, 74],
        vec![79, 60, 97],
        vec![74],
    ]
}

fn monkeys() -> Vec<Monkey> {
    vec![
        Monkey::new(
            |item: Item| item * 3,
            |item: Item| {
                if item % 5 == 0 {
                    2
                } else {
                    7
                }
            },
        ),
        Monkey::new(
            |item: Item| item + 7,
            |item: Item| {
                if item % 2 == 0 {
                    3
                } else {
                    6
                }
            },
        ),
        Monkey::new(
            |item: Item| item + 5,
            |item: Item| {
                if item % 13 == 0 {
                    5
                } else {
                    4
                }
            },
        ),
        Monkey::new(
            |item: Item| item + 8,
            |item: Item| {
                if item % 19 == 0 {
                    6
                } else {
                    0
                }
            },
        ),
        Monkey::new(
            |item: Item| item + 4,
            |item: Item| {
                if item % 11 == 0 {
                    3
                } else {
                    1
                }
            },
        ),
        Monkey::new(
            |item: Item| item * 2,
            |item: Item| {
                if item % 3 == 0 {
                    4
                } else {
                    1
                }
            },
        ),
        Monkey::new(
            |item: Item| item + 6,
            |item: Item| {
                if item % 7 == 0 {
                    7
                } else {
                    0
                }
            },
        ),
        Monkey::new(
            |item: Item| item * item,
            |item: Item| {
                if item % 17 == 0 {
                    2
                } else {
                    5
                }
            },
        ),
    ]
}

fn items() -> Vec<Vec<Item>> {
    vec![
        vec![78, 53, 89, 51, 52, 59, 58, 85],
        vec![64],
        vec![71, 93, 65, 82],
        vec![67, 73, 95, 75, 56, 74],
        vec![85, 91, 90],
        vec![67, 96, 69, 55, 70, 83, 62],
        vec![53, 86, 98, 70, 64],
        vec![88, 64],
    ]
}

#[test]
fn test() {
    let sol1_example = sol1(&mut example_monkeys(), &mut example_items());
    assert_eq!(sol1_example, 10605);

    let sol1 = sol1(&mut monkeys(), &mut items());
    assert_eq!(sol1, 50616);

    let sol2_example = sol2(&mut example_monkeys(), &mut example_items(), 96577);
    assert_eq!(sol2_example, 2713310158);

    let sol2 = sol2(&mut monkeys(), &mut items(), 9699690);
    assert_eq!(sol2, 11309046332);
}
