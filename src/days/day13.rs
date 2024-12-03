use crate::{Solution, SolutionPair};

use std::cmp::Ordering;
use std::fmt::{self};
use std::{fs::read_to_string, str::FromStr, string::ParseError};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day13.txt").unwrap();
    let parsed: Vec<Group> = input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.parse().unwrap())
        .collect();

    let sol1: u64 = sol1(&parsed);
    let sol2: u64 = sol2(&parsed);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn sol1(input: &Vec<Group>) -> u64 {
    let pairs = input.chunks(2);
    let mut in_right_order: Vec<i32> = vec![];
    for (i, pair) in pairs.enumerate() {
        match is_in_right_order(&pair[0], &pair[1]) {
            Ordered::OK => {
                in_right_order.push(i as i32 + 1);
            }
            _ => (),
        }
    }

    in_right_order.iter().sum::<i32>() as u64
}

fn sol2(input: &Vec<Group>) -> u64 {
    let mut sorted = input.clone();
    let two = &Group::List(vec![Group::List(vec![Group::Int(2)])]);
    let six = &Group::List(vec![Group::List(vec![Group::Int(6)])]);
    sorted.push(two.clone());
    sorted.push(six.clone());

    sorted.sort_by(|a, b| match is_in_right_order(a, b) {
        Ordered::OK => Ordering::Less,
        Ordered::WRONG => Ordering::Equal,
        Ordered::SAME => Ordering::Equal,
    });

    let mut answers = vec![];

    for (i, group) in sorted.iter().enumerate() {
        if group == two || group == six {
            answers.push(i as u64 + 1);
        }
    }

    answers.into_iter().reduce(|acc, item| item * acc).unwrap()
}

///////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq)]
enum Group {
    List(Vec<Group>),
    Int(i32),
}

enum Ordered {
    OK,
    WRONG,
    SAME,
}

impl From<String> for Group {
    fn from(v: String) -> Group {
        let mut result: Vec<Group> = vec![];
        let mut tmp = String::new();
        let mut nest = -1;

        if v != "" && !v.contains(',') && !v.contains('[') {
            let int: i32 = v.parse().unwrap();
            return Group::Int(int);
        }

        for c in v.chars() {
            if c == '[' {
                nest += 1;
                if nest > 0 {
                    tmp += &c.to_string()
                }
            } else if c == ']' {
                nest -= 1;
                if nest >= 0 {
                    tmp += &c.to_string()
                }
                if nest <= 0 && tmp.len() > 0 {
                    result.push(Self::from(tmp));
                    tmp = String::new();
                }
            } else if c == ',' && nest <= 0 && tmp.len() > 0 {
                result.push(Self::from(tmp));
                tmp = String::new();
            } else if c == ',' && nest == 0 {
                //
            } else if nest >= 0 {
                tmp += &c.to_string();
            }
        }

        Group::List(result)
    }
}

impl FromStr for Group {
    type Err = ParseError;

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        Ok(Group::from(v.to_string()))
    }
}

impl Clone for Group {
    fn clone(&self) -> Self {
        match self {
            Group::Int(i) => Group::Int(*i),
            Group::List(l) => Group::List(l.clone()),
        }
    }
}

impl fmt::Debug for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Group::Int(v) => f.write_str(v.to_string().as_str()),
            Group::List(v) => f.debug_list().entries(v).finish(),
        }
    }
}

fn is_in_right_order(left: &Group, right: &Group) -> Ordered {
    // println!("Compare {:?} vs {:?}", left, right);
    match (left, right) {
        (&Group::Int(l), &Group::Int(r)) if l < r => Ordered::OK,
        (&Group::Int(l), &Group::Int(r)) if l > r => Ordered::WRONG,
        (&Group::Int(_), &Group::Int(_)) => Ordered::SAME,
        (Group::List(l), Group::List(r)) => {
            for i in 0..l.len() {
                if i >= r.len() {
                    return Ordered::WRONG;
                }
                match is_in_right_order(&l[i], &r[i]) {
                    Ordered::SAME => continue,
                    ordered => return ordered,
                }
            }

            if l.len() < r.len() {
                Ordered::OK
            } else {
                Ordered::SAME
            }
        }
        (Group::List(_), Group::Int(_)) => {
            is_in_right_order(left, &Group::List(vec![right.clone()]))
        }
        (Group::Int(_), Group::List(_)) => {
            is_in_right_order(&Group::List(vec![left.clone()]), right)
        }
    }
}
