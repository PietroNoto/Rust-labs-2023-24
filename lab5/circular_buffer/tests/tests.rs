mod tests
{
    use circular_buffer::CircularBuffer;

    #[test]
    fn test_write()
    {
        let mut cb = CircularBuffer::<u32>::new(10);
        assert!(cb.write(2).is_ok());
        assert_eq!(cb.size(), 1);
    }

    #[test]
    fn test_read()
    {
        let mut cb = CircularBuffer::<u32>::new(10);
        let mut el = cb.read();
        assert!(el.is_none());
        assert!(!cb.write(2).is_err());
        el = cb.read();
        assert_eq!(el, Some(2));
        assert_eq!(cb.size(), 0);
    }

    #[test]
    fn clear_buffer()
    {
        let mut cb = CircularBuffer::<u32>::new(10);
        assert!(!cb.write(1).is_err());
        assert!(!cb.write(2).is_err());
        assert!(!cb.write(3).is_err());

        cb.clear();
        assert_eq!(cb.size(), 0);
        assert!(cb.read().is_none());
    }


    #[test]
    fn full_buffer()
    {
        let mut cb = CircularBuffer::<u32>::new(4);
        for i in 0..4
        {
            assert!(!cb.write(i).is_err());
        }
        assert!(cb.write(11).is_err());
    }

    #[test]
    fn read_after_write_after_full()
    {
        let mut cb = CircularBuffer::<u32>::new(4);
        for i in 0..4
        {
            assert!(!cb.write(i).is_err());
        }
        cb.read();
        cb.write(4);
        assert_eq!(cb.read(), Some(1))
    }

    #[test]
    fn empty_buffer()
    {
        let mut cb = CircularBuffer::<u32>::new(4);
        
        assert!(cb.read().is_none());
    }

    #[test]
    fn overwrite()
    {
        let mut cb = CircularBuffer::<u32>::new(4);
        for i in 0..4
        {
            assert!(!cb.write(i).is_err());
        }
        cb.overwrite(4);
        assert_ne!(cb.read(), Some(4));
        assert_eq!(cb.read(), Some(2));
        cb.write(6);
    }

    #[test]
    fn make_contiguous()
    {
        let mut cb = CircularBuffer::<u32>::new(4);
        for i in 0..4
        {
            assert!(!cb.write(i).is_err());
        }
        cb.overwrite(4);
        cb.overwrite(5);
        cb.read();
        cb.make_contiguous();

        assert_eq!(cb.read(), Some(3));
        assert_eq!(cb.read(), Some(4));
        assert_eq!(cb.read(), Some(5));
        assert_eq!(cb.read(), None);
    }

    #[test]
    fn test_index()
    {
        let mut cb = CircularBuffer::<u32>::new(4);
        for i in 0..4
        {
            assert!(!cb.write(i).is_err());
        }
        assert_eq!(cb[0], Some(0));
        cb.overwrite(4);
        cb.overwrite(5);
        cb.read();
        assert_eq!(cb[0], Some(3));
    }
}