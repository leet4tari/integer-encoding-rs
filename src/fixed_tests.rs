#[cfg(test)]
mod tests {
    use fixed::FixedInt;

    use reader::FixedIntReader;
    use writer::FixedIntWriter;

    #[test]
    fn test_u32_enc() {
        let result = (32 as u32).encode_fixed_vec();
        assert_eq!(result, vec![32, 0, 0, 0]);
    }
    #[test]
    fn test_u16_enc() {
        let result = (256 as u16).encode_fixed_vec();
        assert_eq!(result, vec![0, 1]);
    }
    #[test]
    fn test_i16_enc() {
        let result = (-32768 as i16).encode_fixed_vec();
        assert_eq!(result, vec![0, 128]);
    }
    #[test]
    fn test_i32_enc() {
        let result = (-32767 as i32).encode_fixed_vec();
        assert_eq!(result, vec![1, 128, 255, 255]);
    }
    #[test]
    fn test_all_identity() {
        let a: u16 = 17;
        let b: u32 = 17;
        let c: u64 = 17;
        let d: i16 = -17;
        let e: i32 = -17;
        let f: i64 = -17;

        assert_eq!(a, FixedInt::decode_fixed_vec(&a.encode_fixed_vec()));
        assert_eq!(b, FixedInt::decode_fixed_vec(&b.encode_fixed_vec()));
        assert_eq!(c, FixedInt::decode_fixed_vec(&c.encode_fixed_vec()));
        assert_eq!(d, FixedInt::decode_fixed_vec(&d.encode_fixed_vec()));
        assert_eq!(e, FixedInt::decode_fixed_vec(&e.encode_fixed_vec()));
        assert_eq!(f, FixedInt::decode_fixed_vec(&f.encode_fixed_vec()));
    }

    #[test]
    fn test_reader_writer() {
        let mut buf = Vec::with_capacity(128);

        let i1: u32 = 123;
        let i2: u32 = 124;
        let i3: u32 = 125;

        assert!(buf.write_fixedint(i1).is_ok());
        assert!(buf.write_fixedint(i2).is_ok());
        assert!(buf.write_fixedint(i3).is_ok());

        assert_eq!(3 * 4, buf.len());

        let mut reader: &[u8] = buf.as_ref();

        let i1_res = reader.read_fixedint().unwrap();
        let i2_res = reader.read_fixedint().unwrap();
        let i3_res = reader.read_fixedint().unwrap();

        assert_eq!(i1, i1_res);
        assert_eq!(i2, i2_res);
        assert_eq!(i3, i3_res);
    }
}
