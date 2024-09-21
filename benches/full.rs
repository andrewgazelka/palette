use divan::{Bencher, black_box};
use rand::prelude::*;

fn main() {
    divan::main();
}

const SAMPLE_SIZES: &[usize] = &[10, 100, 1000, 4096];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BlockState(u16);

use palette::{PalettedContainer, BLOCKS_PER_SECTION};

#[divan::bench(args = SAMPLE_SIZES)]
fn set_single(bencher: Bencher, size: usize) {
    bencher.counter(size).bench(|| {
        let mut container = PalettedContainer::Single(BlockState(0));
        for i in 0..size {
            container.set(i % BLOCKS_PER_SECTION, BlockState(0));
        }
    });
}

#[divan::bench(args = SAMPLE_SIZES)]
fn set_indirect(bencher: Bencher, size: usize) {
    bencher.counter(size).bench(|| {
        let mut container = PalettedContainer::Single(BlockState(0));
        for i in 0..16 {
            container.set(i, BlockState(i as u16));
        }
        for i in 0..size {
            container.set(i % BLOCKS_PER_SECTION, BlockState((i % 16) as u16));
        }
    });
}

#[divan::bench(args = SAMPLE_SIZES)]
fn set_direct(bencher: Bencher, size: usize) {
    bencher.counter(size).bench(|| {
        let mut container = PalettedContainer::Single(BlockState(0));
        for i in 0..BLOCKS_PER_SECTION {
            container.set(i, BlockState(i as u16));
        }
        for i in 0..size {
            container.set(i % BLOCKS_PER_SECTION, BlockState((i % 256) as u16));
        }
    });
}

#[divan::bench(args = SAMPLE_SIZES)]
fn get_single(bencher: Bencher, size: usize) {
    let container = PalettedContainer::Single(BlockState(42));
    bencher.counter(size).bench(|| {
        for i in 0..size {
            black_box(container.get_unchecked(i % BLOCKS_PER_SECTION));
        }
    });
}

#[divan::bench(args = SAMPLE_SIZES)]
fn get_indirect(bencher: Bencher, size: usize) {
    let mut container = PalettedContainer::Single(BlockState(0));
    for i in 0..16 {
        container.set(i, BlockState(i as u16));
    }
    bencher.counter(size).bench(|| {
        for i in 0..size {
            black_box(container.get_unchecked(i));
        }
    });
}

#[divan::bench(args = SAMPLE_SIZES)]
fn get_direct(bencher: Bencher, size: usize) {
    let mut container = PalettedContainer::Single(BlockState(0));
    for i in 0..BLOCKS_PER_SECTION {
        container.set(i, BlockState(i as u16));
    }
    bencher.counter(size).bench(|| {
        for i in 0..size {
            black_box(container.get_unchecked(i % BLOCKS_PER_SECTION));
        }
    });
}

#[divan::bench(args = SAMPLE_SIZES)]
fn upgrade_single_to_indirect(bencher: Bencher, size: usize) {
    bencher.counter(size).bench(|| {
        for _ in 0..size {
            let mut container = PalettedContainer::Single(BlockState(0));
            container.set(0, BlockState(1));
            black_box(&container);
        }
    });
}

#[divan::bench(args = SAMPLE_SIZES)]
fn upgrade_indirect_to_direct(bencher: Bencher, size: usize) {
    bencher.counter(size).bench(|| {
        for _ in 0..size {
            let mut container = PalettedContainer::Single(BlockState(0));
            for i in 0..16 {
                container.set(i, BlockState(i as u16));
            }
            container.set(16, BlockState(16));
            black_box(&container);
        }
    });
}

// #[divan::bench(args = SAMPLE_SIZES)]
// fn random_access(bencher: Bencher, size: usize) {
//     let mut rng = StdRng::seed_from_u64(42);
//     let mut container = PalettedContainer::Single(BlockState(0));
// 
//     // Prepare a mix of Single, Indirect, and Direct containers
//     for i in 0..BLOCKS_PER_SECTION {
//         if i < 16 {
//             container.set(i, BlockState(i as u16));
//         } else if i < 32 {
//             container.set(i, BlockState((i % 16) as u16));
//         } else {
//             container.set(i, BlockState(rng.gen::<u16>()));
//         }
//     }
// 
//     bencher.counter(size).bench(|| {
//         for _ in 0..size {
//             let index = rng.gen_range(0..BLOCKS_PER_SECTION);
//             if rng.gen_bool(0.5) {
//                 black_box(container.get(index));
//             } else {
//                 container.set(index, BlockState(rng.gen::<u16>()));
//             }
//         }
//     });
// }