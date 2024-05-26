use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

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

    let mut initial = BTreeSet::new();
    let mut numbers = BTreeMap::<Cost, Vec<N>>::new();
    let mut seen = BitVec::<u32>::repeat(false, 1 << 32).into_boxed_bitslice();
    for arg in args {
        let (n, cost) = if let Some((n, cost)) = arg.split_once('=') {
            (n, cost.parse().context("bad number cost")?)
        } else {
            (&arg[..], 1)
        };

        ensure!(cost > 0, "bad number cost");

        let n = n.parse().context("bad number")?;

        initial.insert(n);
        numbers.entry(cost).or_default().push(n);
        seen.set(idx(n), true);
    }

    for new_cost in 1.. {
        let mut new_numbers = numbers.remove(&new_cost).unwrap_or_default();

        println!("searching for cost {new_cost}");

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

        for i in 1..new_cost / 2 + 1 {
            let j = new_cost - i;

            let a_slice = numbers.get(&i).unwrap().as_slice();
            let b_slice = numbers.get(&j).unwrap().as_slice();

            for &a in a_slice {
                for &b in b_slice {
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
            fn print_tree(
                n: N,
                cost: Cost,
                depth: usize,
                breakdown: &mut impl FnMut(N, Cost) -> Option<(N, Cost, N, Cost, Op)>,
            ) {
                if let Some((lhs, lcost, rhs, rcost, op)) = breakdown(n, cost) {
                    println!("{}{lhs} {op} {rhs}", "  ".repeat(depth));
                    print_tree(lhs, lcost, depth + 1, breakdown);
                    if lhs != rhs {
                        print_tree(rhs, rcost, depth + 1, breakdown);
                    }
                }
            }

            println!("found target number {target}:");
            print_tree(target, new_cost, 1, &mut |t, cost| {
                if !initial.contains(&t) {
                    for j in (1..cost / 2 + 1).rev() {
                        let i = cost - j;

                        let a_slice = numbers.get(&i).unwrap().as_slice();
                        let b_slice = numbers.get(&j).unwrap().as_slice();

                        for &a in a_slice {
                            for &b in b_slice {
                                for (n, (swap, op)) in [
                                    (a.saturating_add(b), (false, Op::Add)),
                                    (a.saturating_mul(b), (false, Op::Mul)),
                                    (a.saturating_sub(b), (false, Op::Sub)),
                                    (b.saturating_sub(a), (true, Op::Sub)),
                                    (div(a, b), (false, Op::Div)),
                                    (div(b, a), (true, Op::Div)),
                                    (pow(a, b), (false, Op::Pow)),
                                    (pow(b, a), (true, Op::Pow)),
                                ] {
                                    if n == t {
                                        return Some(if swap {
                                            (b, j, a, i, op)
                                        } else {
                                            (a, i, b, j, op)
                                        });
                                    }
                                }
                            }
                        }
                    }
                }

                None
            });

            break;
        }

        new_numbers.shrink_to_fit();
        numbers.insert(new_cost, new_numbers);
    }

    Ok(())
}
