extern crate bencher;

use bencher::{Bencher, benchmark_group, benchmark_main};
use tulip::BloomFilter;

fn build_filter<T>(size: usize, rate: f64) -> (BloomFilter<T>, usize, f64) { 
    let f: BloomFilter<T> = BloomFilter::new(size, rate);

    (f, size, rate)
}

fn base_insert(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(10, 0.1);
    bench.iter(|| { f.0.insert(&f.1) });
}

fn base_contains(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(10, 0.1);
    f.0.insert(&f.1);

    bench.iter(|| { f.0.contains(&f.1) });
}

fn base_fill_contains(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(10, 0.1);

    bench.iter(|| { 
        for i in 0..=f.1 { 
            f.0.insert(&i);
        }

        f.0.contains(&f.1);
    });
}

fn scaled_pfr_100(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(100, 0.01);
    bench.iter(|| { f.0.insert(&f.1) });
}

fn scaled_pfr_contains_100(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(100, 0.01);
    f.0.insert(&f.1);

    bench.iter(|| { f.0.contains(&f.1) });
}


fn base_fill_contains_100(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(100, 0.01);

    bench.iter(|| { 
        for i in 0..=f.1 { 
            f.0.insert(&i);
        }

        f.0.contains(&f.1);
    });
}

fn scaled_pfr_1000(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(1000, 0.001);
    bench.iter(|| { f.0.insert(&f.1) });
}

fn scaled_pfr_contains_1000(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(1000, 0.001);
    f.0.insert(&f.1);

    bench.iter(|| { f.0.contains(&f.1) });
}

fn base_fill_contains_1000(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(1000, 0.001);

    bench.iter(|| { 
        for i in 0..=f.1 { 
            f.0.insert(&i);
        }

        f.0.contains(&f.1);
    });
}

fn scaled_pfr_10000(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(10000, 0.0001);
    bench.iter(|| { f.0.insert(&f.1) });
}

fn scaled_pfr_contains_10000(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(10000, 0.0001);
    f.0.insert(&f.1);

    bench.iter(|| { f.0.contains(&f.1) });
}

fn base_fill_contains_10000(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(1000, 0.0001);

    bench.iter(|| { 
        for i in 0..=f.1 { 
            f.0.insert(&i);
        }

        f.0.contains(&f.1);
    });
}

fn scaled_pfr_100000(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(100000, 0.00001);
    bench.iter(|| { f.0.insert(&f.1) });
}

fn scaled_pfr_contains_100000(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(100000, 0.00001);
    f.0.insert(&f.1);

    bench.iter(|| { f.0.contains(&f.1) });
}

fn base_fill_contains_100000(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(100000, 0.00001);

    bench.iter(|| { 
        for i in 0..=f.1 { 
            f.0.insert(&i);
        }

        f.0.contains(&f.1);
    });
}

benchmark_group!(benches, base_insert,
                 base_contains,
                 scaled_pfr_100,
                 scaled_pfr_contains_100,
                 scaled_pfr_1000,
                 scaled_pfr_contains_1000,
                 scaled_pfr_10000,
                 scaled_pfr_contains_10000,
                 scaled_pfr_100000,
                 scaled_pfr_contains_100000,
                 base_fill_contains,
                 base_fill_contains_100,
                 base_fill_contains_1000,
                 base_fill_contains_10000,
                 base_fill_contains_100000);

benchmark_main!(benches);
