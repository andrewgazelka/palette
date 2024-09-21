use palette::{Indirect, PalettedContainer, BLOCKS_PER_SECTION};
use rand::prelude::*;
use tango_bench::{benchmark_fn, tango_benchmarks, tango_main, IntoBenchmarks};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BlockState(u16);

fn generate_random_blockstates(count: usize) -> &'static [BlockState] {
    let mut rng = StdRng::seed_from_u64(42);
    let states: Box<[_]> = (0..count).map(|_| BlockState(rng.gen())).collect();
    Box::leak(states)
}

fn bench_set() -> impl IntoBenchmarks {
    let blockstates = generate_random_blockstates(BLOCKS_PER_SECTION);

    vec![
        benchmark_fn("set/single", move |b| {
            let mut container = PalettedContainer::Single(BlockState(0));
            let mut index = 0;

            b.iter(move || {
                container.set(index, blockstates[index]);
                index = (index + 1) % BLOCKS_PER_SECTION;
            })
        }),
        benchmark_fn("set/indirect", move |b| {
            let mut container = PalettedContainer::Indirect(Indirect::from_single(BlockState(0)));
            let mut index = 0;
            b.iter(move || {
                container.set(index, blockstates[index]);
                index = (index + 1) % BLOCKS_PER_SECTION;
            })
        }),
        benchmark_fn("set/direct", move |b| {
            let mut container = PalettedContainer::Direct(vec![BlockState(0); BLOCKS_PER_SECTION]);
            let mut index = 0;
            b.iter(move || {
                container.set(index, blockstates[index]);
                index = (index + 1) % BLOCKS_PER_SECTION;
            })
        }),
    ]
}

fn bench_get() -> impl IntoBenchmarks {
    let blockstates = generate_random_blockstates(BLOCKS_PER_SECTION);

    vec![
        benchmark_fn("get/single", move |b| {
            let container = PalettedContainer::Single(BlockState(42));
            let mut index = 0;
            b.iter(move || {
                let _ = container.get(index);
                index = (index + 1) % BLOCKS_PER_SECTION;
            })
        }),
        benchmark_fn("get/indirect", move |b| {
            let mut container = PalettedContainer::Indirect(Indirect::from_single(BlockState(0)));
            for (i, state) in blockstates.iter().enumerate() {
                container.set(i, *state);
            }
            let mut index = 0;
            b.iter(move || {
                let _ = container.get(index);
                index = (index + 1) % BLOCKS_PER_SECTION;
            })
        }),
        benchmark_fn("get/direct", move |b| {
            let container = PalettedContainer::Direct(blockstates.to_vec());
            let mut index = 0;
            b.iter(move || {
                let _ = container.get(index);
                index = (index + 1) % BLOCKS_PER_SECTION;
            })
        }),
    ]
}

tango_benchmarks!(bench_set(), bench_get());

tango_main!();
