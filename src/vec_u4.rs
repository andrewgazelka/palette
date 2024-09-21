#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VecU4 {
    inner: Vec<u8>,
}

impl VecU4 {
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn zeroed(len: usize) -> Self {
        let len = len / 2;
        Self {
            inner: vec![0_u8; len],
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity((capacity + 1) / 2),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.inner.iter().flat_map(|x| {
            let lower = x & 0b1111;
            let upper = (x >> 4) & 0b1111_u8;

            [lower, upper]
        })
    }

    pub fn push(&mut self, value: u8) {
        assert!(value <= 0xF, "Value must be 4 bits (0-15)");

        let idx = self.len() / 2;
        if self.len() % 2 == 0 {
            self.inner.push(value);
        } else {
            self.inner[idx] |= value << 4;
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.is_empty() {
            None
        } else {
            let idx = (self.len() - 1) / 2;
            let value = if self.len() % 2 == 0 {
                let v = self.inner[idx] >> 4;
                self.inner.pop();
                v
            } else {
                self.inner[idx] & 0xF
            };
            Some(value)
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len() * 2 - usize::from(self.inner.last().map_or(false, |&x| x <= 0xF))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        if index >= self.len() {
            None
        } else {
            let byte_index = index / 2;
            let nibble = self.inner[byte_index];
            Some(if index % 2 == 0 {
                nibble & 0xF
            } else {
                nibble >> 4
            })
        }
    }

    pub fn set(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        if index >= self.len() {
            return Err("Index out of bounds");
        }
        if value > 0xF {
            return Err("Value must be 4 bits (0-15)");
        }

        let byte_index = index / 2;
        let nibble = &mut self.inner[byte_index];
        if index % 2 == 0 {
            *nibble = (*nibble & 0xF0) | value;
        } else {
            *nibble = (*nibble & 0x0F) | (value << 4);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut vec = VecU4::new();
        vec.push(5);
        vec.push(10);
        vec.push(15);

        assert_eq!(vec.pop(), Some(15));
        assert_eq!(vec.pop(), Some(10));
        assert_eq!(vec.pop(), Some(5));
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut vec = VecU4::new();
        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);

        vec.push(1);
        assert!(!vec.is_empty());
        assert_eq!(vec.len(), 1);

        vec.push(2);
        assert_eq!(vec.len(), 2);

        vec.pop();
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn test_get_and_set() {
        let mut vec = VecU4::new();
        vec.push(3);
        vec.push(7);
        vec.push(11);
        vec.push(15);

        assert_eq!(vec.get(0), Some(3));
        assert_eq!(vec.get(1), Some(7));
        assert_eq!(vec.get(2), Some(11));
        assert_eq!(vec.get(3), Some(15));
        assert_eq!(vec.get(4), None);

        assert!(vec.set(0, 1).is_ok());
        assert!(vec.set(1, 5).is_ok());
        assert!(vec.set(2, 9).is_ok());
        assert!(vec.set(3, 13).is_ok());
        assert!(vec.set(4, 1).is_err());

        assert_eq!(vec.get(0), Some(1));
        assert_eq!(vec.get(1), Some(5));
        assert_eq!(vec.get(2), Some(9));
        assert_eq!(vec.get(3), Some(13));
    }

    #[test]
    #[should_panic(expected = "Value must be 4 bits (0-15)")]
    fn test_push_invalid_value() {
        let mut vec = VecU4::new();
        vec.push(16);
    }

    #[test]
    fn test_clear() {
        let mut vec = VecU4::new();
        vec.push(1);
        vec.push(2);
        vec.clear();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_with_capacity() {
        let vec = VecU4::with_capacity(10);
        assert!(vec.inner.capacity() >= 5);
    }
}
