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

fn scaled_pfr_100(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(10, 0.001);
    bench.iter(|| { f.0.insert(&f.1) });
}

fn scaled_pfr_contains_100(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(1000, 0.001);
    f.0.insert(&f.1);

    bench.iter(|| { f.0.contains(&f.1) });
}

fn scaled_pfr_1000(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(10, 0.0001);
    bench.iter(|| { f.0.insert(&f.1) });
}

fn scaled_pfr_contains_1000(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(10, 0.0001);
    f.0.insert(&f.1);

    bench.iter(|| { f.0.contains(&f.1) });
}

fn scaled_pfr_10000(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(10, 0.00001);
    bench.iter(|| { f.0.insert(&f.1) });
}

fn scaled_pfr_contains_10000(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(10, 0.00001);
    f.0.insert(&f.1);

    bench.iter(|| { f.0.contains(&f.1) });
}

fn scaled_pfr_100000(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(10, 0.000001);
    bench.iter(|| { f.0.insert(&f.1) });
}

fn scaled_pfr_contains_100000(bench: &mut Bencher) { 
    let mut f = build_filter::<usize>(10, 0.000001);
    f.0.insert(&f.1);

    bench.iter(|| { f.0.contains(&f.1) });
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
                 scaled_pfr_contains_100000);

benchmark_main!(benches);
