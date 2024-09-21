#![feature(allocator_api)]
#![feature(impl_trait_in_assoc_type)]

pub use crate::indirect::Indirect;
use more_asserts::debug_assert_lt;

// 4096
pub const BLOCKS_PER_SECTION: usize = 16 * 16 * 16;

// 4 bits per block for 16 block

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PalettedContainer<T> {
    Single(T),
    Indirect(Indirect<T>),
    Direct(Vec<T>),
}

impl<T: Copy> PalettedContainer<T> {
    fn upgrade(&mut self) {
        // todo: is there a better way to do this? perhaps std::mem::zeroed is UB in some cases?
        *self = match core::mem::replace(self, Self::Single(unsafe { std::mem::zeroed() })) {
            Self::Single(value) => {
                let indirect = Indirect::from_single(value);
                Self::Indirect(indirect)
            }
            Self::Indirect(indirect) => {
                let direct = indirect.iter().copied().collect();
                Self::Direct(direct)
            }
            this @ Self::Direct(..) => {
                // no-op
                this
            }
        };
    }
}

impl<T: PartialEq + Copy> PalettedContainer<T> {
    pub fn set(&mut self, index: usize, value: T) {
        debug_assert_lt!(index, BLOCKS_PER_SECTION);

        match self {
            Self::Single(current) => {
                if value == *current {
                    return;
                }

                self.upgrade();
                self.set(index, value);
            }
            Self::Indirect(indirect) => {
                if indirect.set(index, value).is_ok() {
                    return;
                }

                self.upgrade();
                self.set(index, value);
            }
            Self::Direct(direct) => {
                direct[index] = value;
            }
        }
    }

    pub fn get(&self, index: usize) -> &T {
        debug_assert_lt!(index, BLOCKS_PER_SECTION);

        match self {
            Self::Single(current) => current,
            Self::Indirect(indirect) => indirect.get(index),
            Self::Direct(direct) => &direct[index],
        }
    }
}

mod indirect;
mod vec_u4;
