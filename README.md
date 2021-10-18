# lupine 
A Bloom Filter using Kirsch and Mitzenmacher optimization with two hash functions.

Hash: [Fx Hash](https://github.com/cbreeden/fxhash)


## Use

```
use lupine::BloomFilter;

fn main() {
    let mut filter = BloomFilter::new(1_000, 0.001);
    filter.insert(&897);

    let contains = filter.contains(&897);
    let does_not_contain = filter.contains(&100);
}
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
