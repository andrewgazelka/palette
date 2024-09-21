#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArrayU4 {
    inner: Box<[u8]>,
}

impl ArrayU4 {
    pub fn zeroed(len: usize) -> Self {
        // assert divisible by 2
        debug_assert_eq!(len % 2, 0);

        let len = len / 2;
        Self {
            inner: vec![0_u8; len].into_boxed_slice(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.inner.iter().flat_map(|x| {
            let lower = x & 0b1111;
            let upper = (x >> 4) & 0b1111_u8;

            [lower, upper]
        })
    }

    pub const fn len(&self) -> usize {
        self.inner.len() * 2
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        if index >= self.len() {
            return None;
        }
        
        unsafe { Some(self.get_unchecked(index)) }
    }
    
    pub unsafe fn get_unchecked(&self, index: usize) -> u8 {
        let byte_index = index >> 1;  // Equivalent to index / 2
        let nibble = *self.inner.get_unchecked(byte_index);
        (nibble >> ((index & 1) << 2)) & 0xF
    }


    pub fn set(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        if index >= self.len() {
            return Err("Index out of bounds");
        }
        if value > 0xF {
            return Err("Value must be 4 bits (0-15)");
        }
        unsafe { self.set_unchecked(index, value); }
        Ok(())
    }

    pub unsafe fn set_unchecked(&mut self, index: usize, value: u8) {
        let byte_index = index >> 1;  // Equivalent to index / 2
        let nibble = self.inner.get_unchecked_mut(byte_index);
        let shift = (index & 1) << 2;  // 0 for even indices, 4 for odd indices
        let mask = 0xF << shift;
        *nibble = (*nibble & !mask) | ((value & 0xF) << shift);
    }
}
