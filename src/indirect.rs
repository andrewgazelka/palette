use crate::vec_u4::ArrayU4;
use arrayvec::ArrayVec;

pub enum IndirectIndexError {
    Full,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Indirect<T> {
    palette: ArrayVec<T, 16>,
    elements: ArrayU4,
}

impl<T> Indirect<T> {
    pub fn from_single(element: T) -> Self {
        let mut palette = ArrayVec::new();
        palette.push(element);

        Self {
            palette,
            elements: ArrayU4::zeroed(4096),
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
        unsafe { self.palette.push_unchecked(element) };

        unsafe { self.elements.set_unchecked(index, palette_index as u8) };

        Ok(())
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        let palette_index = unsafe { self.elements.get_unchecked(index) };
        unsafe { self.palette.get_unchecked(palette_index as usize) }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let palette_index = self.elements.get(index)?;
        Some(unsafe { self.palette.get_unchecked(palette_index as usize) })
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
