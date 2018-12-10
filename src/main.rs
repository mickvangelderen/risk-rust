#[derive(Debug)]
struct Round1v1 {
    a0d1: f64,
    a1d0: f64,
}

#[derive(Debug)]
struct Round2v2 {
    a0d2: f64,
    a1d1: f64,
    a2d0: f64,
}

fn compute_a1d1() -> Round1v1 {
    let mut a0d1 = 0u64;
    let mut a1d0 = 0u64;
    for a in 1..=6 {
        for d in 1..=6 {
            if a > d {
                a0d1 += 1;
            } else {
                a1d0 += 1;
            }
        }
    }
    const N: u64 = 6 * 6;
    assert_eq!(a0d1 + a1d0, N);
    Round1v1 {
        a0d1: a0d1 as f64 / N as f64,
        a1d0: a1d0 as f64 / N as f64,
    }
}

fn compute_a2d1() -> Round1v1 {
    let mut a0d1 = 0u64;
    let mut a1d0 = 0u64;
    for a in 1..=6 {
        for b in a..=6 {
            let n_ab = if a == b { 1 } else { 2 };
            for d in 1..=6 {
                let n = n_ab;

                if b > d {
                    a0d1 += n;
                } else {
                    a1d0 += n;
                }
            }
        }
    }
    const N: u64 = 6 * 6 * 6;
    assert_eq!(a0d1 + a1d0, N);
    Round1v1 {
        a0d1: a0d1 as f64 / N as f64,
        a1d0: a1d0 as f64 / N as f64,
    }
}

fn compute_a3d1() -> Round1v1 {
    let mut a0d1 = 0u64;
    let mut a1d0 = 0u64;
    for a in 1..=6 {
        for b in a..=6 {
            for c in b..=6 {
                let n_abc = if a == b && b == c {
                    1
                } else if a == b || b == c {
                    3
                } else {
                    6
                };
                for d in 1..=6 {
                    let n = n_abc;

                    if c > d {
                        a0d1 += n;
                    } else {
                        a1d0 += n;
                    }
                }
            }
        }
    }
    const N: u64 = 6 * 6 * 6 * 6;
    assert_eq!(a0d1 + a1d0, N);
    Round1v1 {
        a0d1: a0d1 as f64 / N as f64,
        a1d0: a1d0 as f64 / N as f64,
    }
}

fn compute_a1d2() -> Round1v1 {
    let mut a0d1 = 0u64;
    let mut a1d0 = 0u64;
    for a in 1..=6 {
        for d in 1..=6 {
            for e in d..=6 {
                let n_de = if d == e { 1 } else { 2 };
                let n = n_de;

                if a > d && a > e {
                    a0d1 += n;
                } else {
                    a1d0 += n;
                }
            }
        }
    }
    const N: u64 = 6 * 6 * 6;
    assert_eq!(a0d1 + a1d0, N);
    Round1v1 {
        a0d1: a0d1 as f64 / N as f64,
        a1d0: a1d0 as f64 / N as f64,
    }
}

fn compute_a2d2() -> Round2v2 {
    let mut a0d2 = 0u64;
    let mut a1d1 = 0u64;
    let mut a2d0 = 0u64;
    for a in 1..=6 {
        for b in a..=6 {
            let n_ab = if a == b { 1 } else { 2 };
            for d in 1..=6 {
                for e in d..=6 {
                    let n_de = if d == e { 1 } else { 2 };
                    let n = n_ab * n_de;

                    if b > e {
                        if a > d {
                            a0d2 += n;
                        } else {
                            a1d1 += n;
                        }
                    } else {
                        if a > d {
                            a1d1 += n;
                        } else {
                            a2d0 += n;
                        }
                    }
                }
            }
        }
    }
    const N: u64 = 6 * 6 * 6 * 6;
    assert_eq!(a0d2 + a1d1 + a2d0, N);
    Round2v2 {
        a0d2: a0d2 as f64 / N as f64,
        a1d1: a1d1 as f64 / N as f64,
        a2d0: a2d0 as f64 / N as f64,
    }
}

fn compute_a3d2() -> Round2v2 {
    let mut a0d2 = 0u64;
    let mut a1d1 = 0u64;
    let mut a2d0 = 0u64;
    for a in 1..=6 {
        for b in a..=6 {
            for c in b..=6 {
                let n_abc = if a == b && b == c {
                    1
                } else if a == b || b == c || a == c {
                    3
                } else {
                    6
                };
                for d in 1..=6 {
                    for e in d..=6 {
                        let n_de = if d == e { 1 } else { 2 };
                        let n = n_abc * n_de;

                        if c > e {
                            if b > d {
                                a0d2 += n;
                            } else {
                                a1d1 += n;
                            }
                        } else {
                            if b > d {
                                a1d1 += n;
                            } else {
                                a2d0 += n;
                            }
                        }
                    }
                }
            }
        }
    }
    const N: u64 = 6 * 6 * 6 * 6 * 6;
    assert_eq!(a0d2 + a1d1 + a2d0, N);
    Round2v2 {
        a0d2: a0d2 as f64 / N as f64,
        a1d1: a1d1 as f64 / N as f64,
        a2d0: a2d0 as f64 / N as f64,
    }
}

const N: usize = 20;

#[derive(Default, Debug)]
struct Match {
    /// The probability that a match ends with a(i + 1)d0 is stored in w[i].
    w: [f64; N],
    /// The probability that a match ends with a0d(i + 1) is stored in l[i].
    l: [f64; N],
}

fn main() {
    let a1d1 = compute_a1d1();
    let a2d1 = compute_a2d1();
    let a3d1 = compute_a3d1();
    let a1d2 = compute_a1d2();
    let a2d2 = compute_a2d2();
    let a3d2 = compute_a3d2();

    let mut matches: [[Match; N]; N] = Default::default();

    #[inline]
    fn matches_a0d1_w(matches: &[[Match; N]; N], ai: usize, di: usize, i: usize) -> f64 {
        if di == 0 {
            if ai == i {
                1.0
            } else {
                0.0
            }
        } else {
            matches[ai][di - 1].w[i]
        }
    }

    #[inline]
    fn matches_a0d1_l(matches: &[[Match; N]; N], ai: usize, di: usize, i: usize) -> f64 {
        if di == 0 {
            0.0
        } else {
            matches[ai][di - 1].l[i]
        }
    }

    #[inline]
    fn matches_a1d0_w(matches: &[[Match; N]; N], ai: usize, di: usize, i: usize) -> f64 {
        if ai == 0 {
            0.0
        } else {
            matches[ai - 1][di].w[i]
        }
    }

    #[inline]
    fn matches_a1d0_l(matches: &[[Match; N]; N], ai: usize, di: usize, i: usize) -> f64 {
        if ai == 0 {
            if di == i {
                1.0
            } else {
                0.0
            }
        } else {
            matches[ai - 1][di].l[i]
        }
    }

    #[inline]
    fn matches_a0d2_w(matches: &[[Match; N]; N], ai: usize, di: usize, i: usize) -> f64 {
        assert!(di >= 1);
        if di == 1 {
            if ai == i {
                1.0
            } else {
                0.0
            }
        } else {
            matches[ai][di - 2].w[i]
        }
    }

    #[inline]
    fn matches_a0d2_l(matches: &[[Match; N]; N], ai: usize, di: usize, i: usize) -> f64 {
        assert!(di >= 1);
        if di == 1 {
            0.0
        } else {
            matches[ai][di - 2].l[i]
        }
    }

    #[inline]
    fn matches_a2d0_w(matches: &[[Match; N]; N], ai: usize, di: usize, i: usize) -> f64 {
        assert!(ai >= 1);
        if ai == 1 {
            0.0
        } else {
            matches[ai - 2][di].w[i]
        }
    }

    #[inline]
    fn matches_a2d0_l(matches: &[[Match; N]; N], ai: usize, di: usize, i: usize) -> f64 {
        assert!(ai >= 1);
        if ai == 1 {
            if di == i {
                1.0
            } else {
                0.0
            }
        } else {
            matches[ai - 2][di].l[i]
        }
    }

    for ai in 0..N {
        // # Attackers = ai + 1.
        for di in 0..N {
            // # Defenders = di + 1.
            for i in 0..N {
                // w: # Attackers remaining = i + 1.
                // l: # Defenders remaining = i + 1.
                match ai {
                    0 => match di {
                        0 => {
                            // Attacking with 1, defending with 1.
                            let p = &a1d1;
                            matches[ai][di].w[i] = p.a0d1 * matches_a0d1_w(&matches, ai, di, i)
                                + p.a1d0 * matches_a1d0_w(&matches, ai, di, i);
                            matches[ai][di].l[i] = p.a0d1 * matches_a0d1_l(&matches, ai, di, i)
                                + p.a1d0 * matches_a1d0_l(&matches, ai, di, i);
                        }
                        _ => {
                            // Attacking with 1, defending with 2 or more.
                            let p = &a1d2;
                            matches[ai][di].w[i] = p.a0d1 * matches_a0d1_w(&matches, ai, di, i)
                                + p.a1d0 * matches_a1d0_w(&matches, ai, di, i);
                            matches[ai][di].l[i] = p.a0d1 * matches_a0d1_l(&matches, ai, di, i)
                                + p.a1d0 * matches_a1d0_l(&matches, ai, di, i);
                        }
                    },
                    1 => match di {
                        0 => {
                            // Attacking with 2, defending with 1.
                            let p = &a2d1;
                            matches[ai][di].w[i] = p.a0d1 * matches_a0d1_w(&matches, ai, di, i)
                                + p.a1d0 * matches_a1d0_w(&matches, ai, di, i);
                            matches[ai][di].l[i] = p.a0d1 * matches_a0d1_l(&matches, ai, di, i)
                                + p.a1d0 * matches_a1d0_l(&matches, ai, di, i);
                        }
                        _ => {
                            // Attacking with 2, defending with 2 or more.
                            let p = &a2d2;
                            matches[ai][di].w[i] = p.a0d2 * matches_a0d2_w(&matches, ai, di, i)
                                + p.a1d1 * matches[ai - 1][di - 1].w[i]
                                + p.a2d0 * matches_a2d0_w(&matches, ai, di, i);
                            matches[ai][di].l[i] = p.a0d2 * matches_a0d1_l(&matches, ai, di, i)
                                + p.a1d1 * matches[ai - 1][di - 1].l[i]
                                + p.a2d0 * matches_a2d0_l(&matches, ai, di, i);
                        }
                    },
                    _ => match di {
                        0 => {
                            // Attacking with 3 or more, defending with 1.
                            let p = &a3d1;
                            matches[ai][di].w[i] = p.a0d1 * matches_a0d1_w(&matches, ai, di, i)
                                + p.a1d0 * matches_a1d0_w(&matches, ai, di, i);
                            matches[ai][di].l[i] = p.a0d1 * matches_a0d1_l(&matches, ai, di, i)
                                + p.a1d0 * matches_a1d0_l(&matches, ai, di, i);
                        }
                        _ => {
                            // Attacking with 3 or more, defending with 2 or more.
                            let p = &a3d2;
                            matches[ai][di].w[i] = p.a0d2 * matches_a0d2_w(&matches, ai, di, i)
                                + p.a1d1 * matches[ai - 1][di - 1].w[i]
                                + p.a2d0 * matches_a2d0_w(&matches, ai, di, i);
                            matches[ai][di].l[i] = p.a0d2 * matches_a0d2_l(&matches, ai, di, i)
                                + p.a1d1 * matches[ai - 1][di - 1].l[i]
                                + p.a2d0 * matches_a2d0_l(&matches, ai, di, i);
                        }
                    },
                }
            }
        }
    }

    for ai in 0..N {
        for di in 0..N / 2 {
            // Probability that the attacker wins.
            let mut pw = 0.0;
            // Expected number of troops the attacker loses.
            let mut e_los = 0.0;
            for i in 0..N {
                pw += matches[ai][di].w[i];
            }
            for i in 0..=ai {
                e_los += matches[ai][di].w[i] * ((ai + 1) - (i + 1)) as f64;
            }
            for i in 0..N {
                e_los += matches[ai][di].l[i] * (ai + 1) as f64;
            }
            print!(
                "a{:>02}d{:>02} {:>3.0}% {:>5.2}",
                ai + 1,
                di + 1,
                pw * 100.0,
                e_los
            );
            if di + 1 == N / 2 {
                println!();
            } else {
                print!(" | ");
            }
        }
    }
}
