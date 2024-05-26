use std::{collections::BTreeMap, fmt};

use anyhow::*;
use bitvec::vec::BitVec;

type Cost = u32;
type N = i32;

enum Op {
    Add,
    Mul,
    Sub,
    Div,
    Pow,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Op::Add => "+",
            Op::Mul => "*",
            Op::Sub => "-",
            Op::Div => "/",
            Op::Pow => "^",
        })
    }
}

fn main() -> Result<()> {
    #[inline]
    fn idx(n: N) -> usize {
        n as u32 as usize
    }

    let mut args = std::env::args();

    let target: N = args.nth(1).context("missing target number")?.parse()?;

    let mut seen = BitVec::<u32>::repeat(false, 1 << 32).into_boxed_bitslice();
    let mut numbers = BTreeMap::<Cost, Vec<N>>::new();
    for arg in args {
        let (n, cost) = if let Some((n, cost)) = arg.split_once('=') {
            (n, cost.parse().context("bad number cost")?)
        } else {
            (&arg[..], 1)
        };

        ensure!(cost > 0, "bad number cost");

        let n = n.parse().context("bad number")?;

        numbers.entry(cost).or_default().push(n);
        seen.set(idx(n), true);
    }

    for new_cost in 1.. {
        let mut new_numbers = numbers.remove(&new_cost).unwrap_or_default();

        println!("searching for cost {new_cost}");

        for i in 1..new_cost / 2 + 1 {
            let j = new_cost - i;

            let a_slice = numbers.get(&i).unwrap().as_slice();
            let b_slice = numbers.get(&j).unwrap().as_slice();

            for &a in a_slice {
                for &b in b_slice {
                    #[inline]
                    fn div(a: N, b: N) -> N {
                        if b == 0 {
                            0
                        } else {
                            let res = a.saturating_div(b);
                            if res.saturating_mul(b) != a {
                                0
                            } else {
                                res
                            }
                        }
                    }

                    #[inline]
                    fn pow(a: N, b: N) -> N {
                        if b < 0 {
                            0
                        } else {
                            a.saturating_pow(b as _)
                        }
                    }

                    for n in [
                        a.saturating_add(b),
                        a.saturating_mul(b),
                        a.saturating_sub(b),
                        b.saturating_sub(a),
                        div(a, b),
                        div(b, a),
                        pow(a, b),
                        pow(b, a),
                    ] {
                        let seen = unsafe { seen.get_unchecked_mut(idx(n)) };
                        if !*seen {
                            new_numbers.push(n);
                            seen.commit(true);
                        }
                    }
                }
            }
        }

        if seen[idx(target)] {
            println!("found target number {target} (total cost {new_cost})");

            break;
        }

        numbers.insert(new_cost, new_numbers);

        // if let Some(cost) = costs.get(&target) {
        //     fn print_tree(operations: &BTreeMap<N, (N, N, Op)>, n: &N, depth: usize) {
        //         if let Some((lhs, rhs, op)) = operations.get(n) {
        //             println!("{}{lhs} {op} {rhs}", "  ".repeat(depth));
        //             print_tree(operations, lhs, depth + 1);
        //             if lhs != rhs {
        //                 print_tree(operations, rhs, depth + 1);
        //             }
        //         }
        //     }

        //     println!("found target number {target} (total cost {cost}):");
        //     print_tree(&operations, &target, 1);

        //     return Ok(());
        // }
    }

    Ok(())
}
