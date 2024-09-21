#[cfg(test)]
mod tests {
    use palette::{PalettedContainer, BLOCKS_PER_SECTION};

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct BlockState(u16);

    #[test]
    fn test_paletted_container_single() {
        let mut container = PalettedContainer::Single(BlockState(1));

        assert_eq!(*container.get_unchecked(0), BlockState(1));
        assert_eq!(*container.get_unchecked(BLOCKS_PER_SECTION - 1), BlockState(1));

        // Setting the same value shouldn't change the container type
        container.set(42, BlockState(1));
        assert!(matches!(container, PalettedContainer::Single(_)));

        // Setting a different value should upgrade the container
        container.set(42, BlockState(2));
        assert!(matches!(container, PalettedContainer::Indirect(_)));
    }

    #[test]
    fn test_paletted_container_indirect() {
        let mut container = PalettedContainer::Single(BlockState(1));
        container.set(0, BlockState(2)); // This should upgrade to Indirect

        assert!(matches!(container, PalettedContainer::Indirect(_)));
        assert_eq!(*container.get_unchecked(0), BlockState(2));
        assert_eq!(*container.get_unchecked(1), BlockState(1));

        // Fill the indirect container
        for i in 0..16 {
            container.set(i, BlockState(i as u16));
        }

        // This should cause another upgrade to Direct
        container.set(16, BlockState(16));
        assert!(matches!(container, PalettedContainer::Direct(_)));
    }

    #[test]
    fn test_paletted_container_direct() {
        let mut container = PalettedContainer::Direct(vec![BlockState(0); BLOCKS_PER_SECTION]);

        for i in 0..BLOCKS_PER_SECTION {
            container.set(i, BlockState(i as u16));
        }

        for i in 0..BLOCKS_PER_SECTION {
            assert_eq!(*container.get_unchecked(i), BlockState(i as u16));
        }
    }

    #[test]
    #[should_panic]
    fn test_paletted_container_out_of_bounds() {
        let container = PalettedContainer::Single(BlockState(0));
        container.get_unchecked(BLOCKS_PER_SECTION);
    }
}
