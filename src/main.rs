use std::{
    collections::{btree_map::Entry, BTreeMap},
    fmt,
};

use anyhow::*;

type N = i32;

enum Op {
    Add,
    Mul,
    Sub,
    Div,
    Exp,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Op::Add => "+",
            Op::Mul => "*",
            Op::Sub => "-",
            Op::Div => "/",
            Op::Exp => "^",
        })
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args();

    let target: N = args.nth(1).context("missing target number")?.parse()?;

    let mut numbers = BTreeMap::<u32, Vec<_>>::new();
    let mut costs = args
        .map(|n| {
            let (n, cost) = if let Some((n, cost)) = n.split_once('=') {
                (n, cost.parse().context("bad number cost")?)
            } else {
                (&n[..], 1)
            };

            ensure!(cost > 0, "bad number cost");

            let n = n.parse().context("bad number")?;
            numbers.entry(cost).or_default().push(n);

            Ok((n, cost))
        })
        .collect::<Result<BTreeMap<N, u32>>>()?;

    println!("target: {target}");
    println!("initial numbers:");

    costs
        .iter()
        .for_each(|(n, cost)| println!("  {n} (cost {cost})"));

    let mut operations = BTreeMap::new();

    for new_cost in 1.. {
        println!(
            "searching for cost {new_cost} ({} total numbers reached)",
            costs.len(),
        );

        let new_numbers: BTreeMap<N, (N, N, Op)> = (1..new_cost + 1)
            .flat_map(|i| {
                let j = new_cost - i;

                let numbers = &numbers;
                numbers
                    .get(&i)
                    .into_iter()
                    .flat_map(|vec| vec.iter())
                    .flat_map(move |&lhs| {
                        numbers
                            .get(&j)
                            .into_iter()
                            .flat_map(|vec| vec.iter())
                            .flat_map(move |&rhs| {
                                [
                                    (lhs.saturating_add(rhs), (lhs, rhs, Op::Add)),
                                    (lhs.saturating_mul(rhs), (lhs, rhs, Op::Mul)),
                                    (lhs.saturating_sub(rhs), (lhs, rhs, Op::Sub)),
                                    (
                                        if rhs == 0 {
                                            0
                                        } else {
                                            let res = lhs.saturating_div(rhs);
                                            if res.saturating_mul(rhs) != lhs {
                                                0
                                            } else {
                                                res
                                            }
                                        },
                                        (lhs, rhs, Op::Div),
                                    ),
                                    (
                                        if rhs < 0 {
                                            0
                                        } else {
                                            lhs.saturating_pow(rhs as _)
                                        },
                                        (lhs, rhs, Op::Exp),
                                    ),
                                ]
                            })
                    })
            })
            .collect();

        let curr_cost = numbers.entry(new_cost).or_default();
        new_numbers.into_iter().for_each(|(n, op)| {
            if let Entry::Vacant(v) = costs.entry(n) {
                v.insert(new_cost);
                operations.insert(n, op);
                curr_cost.push(n);
            }
        });

        if let Some(cost) = costs.get(&target) {
            fn print_tree(operations: &BTreeMap<N, (N, N, Op)>, n: &N, depth: usize) {
                if let Some((lhs, rhs, op)) = operations.get(n) {
                    println!("{}{lhs} {op} {rhs}", "  ".repeat(depth));
                    print_tree(operations, lhs, depth + 1);
                    if lhs != rhs {
                        print_tree(operations, rhs, depth + 1);
                    }
                }
            }

            println!("found target number (total cost {cost}):");
            print_tree(&operations, &target, 1);

            return Ok(());
        }
    }

    unreachable!()
}
