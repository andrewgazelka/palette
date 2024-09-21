use crate::vec_u4::VecU4;
use crate::BLOCKS_PER_SECTION;
use arrayvec::ArrayVec;
use more_asserts::debug_assert_lt;

pub enum IndirectIndexError {
    Full,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Indirect<T> {
    palette: ArrayVec<T, 16>,
    elements: VecU4,
}

impl<T> Indirect<T> {
    pub fn from_single(element: T) -> Self {
        let mut palette = ArrayVec::new();
        palette.push(element);

        Self {
            palette,
            elements: VecU4::zeroed(4096),
        }
    }
}

impl<T: PartialEq> Indirect<T> {
    pub fn set(&mut self, index: usize, element: T) -> Result<(), IndirectIndexError> {
        if let Some(palette_index) = self.find_index(&element) {
            self.elements.set(index, palette_index).unwrap();
            return Ok(());
        };

        // we didn't find it, so we need to add it to the palette
        if self.palette.len() == 16 {
            return Err(IndirectIndexError::Full);
        }

        let palette_index = self.palette.len();
        self.palette.push(element);

        self.elements.set(index, palette_index as u8).unwrap();

        Ok(())
    }

    pub fn get(&self, index: usize) -> &T {
        debug_assert_lt!(index, BLOCKS_PER_SECTION);

        let value = self.elements.get(index).unwrap();
        &self.palette[value as usize]
    }

    fn find_index(&self, element: &T) -> Option<u8> {
        let idx = self.palette.iter().position(|x| x == element)?;
        Some(idx as u8)
    }
}

impl<T> Indirect<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elements.iter().map(|x| &self.palette[x as usize])
    }
}
