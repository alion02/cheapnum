use std::{
    collections::{btree_map::Entry, BTreeMap},
    fmt,
};

use anyhow::*;

type N = i64;

enum Op {
    Add,
    Mul,
    Sub,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Op::Add => "+",
            Op::Mul => "*",
            Op::Sub => "-",
        })
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args();

    let target: N = args.nth(1).context("missing target number")?.parse()?;

    let mut costs = args
        .map(|n| {
            let (n, cost) = if let Some((n, cost)) = n.split_once('=') {
                (n, cost.parse().context("bad number cost")?)
            } else {
                (&n[..], 1)
            };

            ensure!(cost > 0, "bad number cost");

            Ok((n.parse().context("bad number")?, cost))
        })
        .collect::<Result<BTreeMap<N, u32>>>()?;

    println!("target: {target}");
    println!("initial numbers:");

    costs
        .iter()
        .for_each(|(n, cost)| println!("  {n} (cost {cost})"));

    let mut operations = BTreeMap::new();

    for i in 1.. {
        for j in 1..i + 1 {
            let new_cost = i + j;

            let new_numbers: BTreeMap<N, (N, N, Op)> = costs
                .iter()
                .filter(|(_, &cost)| (i == cost))
                .flat_map(|(&lhs, _)| {
                    costs
                        .iter()
                        .filter(|(_, &cost)| (j == cost))
                        .flat_map(move |(&rhs, _)| {
                            [
                                (lhs + rhs, (lhs, rhs, Op::Add)),
                                (lhs * rhs, (lhs, rhs, Op::Mul)),
                                (lhs - rhs, (lhs, rhs, Op::Sub)),
                            ]
                        })
                })
                .collect();

            new_numbers.into_iter().for_each(|(n, op)| {
                if let Entry::Vacant(v) = costs.entry(n) {
                    v.insert(new_cost);
                    operations.insert(n, op);
                }
            });

            if let Some(cost) = costs.get(&target) {
                fn print_tree(operations: &BTreeMap<i64, (i64, i64, Op)>, n: &N, depth: usize) {
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
    }

    unreachable!()
}
