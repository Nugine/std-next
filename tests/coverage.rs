use stdx::convert::{self, FromExt, IntoExt, TryFromExt, TryIntoExt};
use stdx::{default, iter, mem, ptr, slice::SliceExt, str::StrExt};

#[test]
fn test_convert_helpers() {
    let value: u8 = 7;
    let from_value: u16 = convert::from(value);
    assert_eq!(from_value, 7);

    let into_value: u16 = convert::into(value);
    assert_eq!(into_value, 7);

    let from_ext: u16 = u16::from_(value);
    assert_eq!(from_ext, 7);

    let into_ext: u16 = value.into_();
    assert_eq!(into_ext, 7);

    let ok: u8 = u8::try_from_(10u16).expect("valid conversion");
    assert_eq!(ok, 10);

    let err: Result<u8, _> = convert::try_from(300u16);
    assert!(err.is_err());

    let ok: u8 = 8u16.try_into_().expect("valid conversion");
    assert_eq!(ok, 8);

    let err: Result<u8, _> = 300u16.try_into_();
    assert!(err.is_err());
}

#[test]
fn test_default_helpers() {
    let default_option: Option<u8> = default::default();
    assert_eq!(default_option, None);

    let customized = default::default_with(|data: &mut Vec<u8>| data.extend([1, 2, 3]));
    assert_eq!(customized, vec![1, 2, 3]);
}

#[test]
fn test_iter_helpers() {
    let doubled: Vec<_> = iter::map_collect([1, 2, 3], |value| value * 2);
    assert_eq!(doubled, vec![2, 4, 6]);

    let filtered: Vec<_> =
        iter::filter_map_collect([1, 2, 3, 4], |value| (value % 2 == 0).then_some(value * 10));
    assert_eq!(filtered, vec![20, 40]);
}

#[test]
fn test_mem_output_size() {
    let size = mem::output_size(&|a: u8, b: u16| -> u32 { u32::from(a) + u32::from(b) });
    assert_eq!(size, core::mem::size_of::<u32>());
}

#[test]
fn test_offset_of_macro() {
    #[repr(C)]
    struct Sample {
        a: u8,
        b: u32,
        c: u16,
    }

    assert_eq!(stdx::offset_of!(Sample, a), 0);
    assert_eq!(stdx::offset_of!(Sample, b), 4);
    assert_eq!(stdx::offset_of!(Sample, c), 8);
}

#[allow(unsafe_code)]
#[test]
fn test_ptr_helpers() {
    let value = 1u32;
    let const_ptr = ptr::cast_ptr::<u32, u32>(&value);
    assert_eq!(const_ptr, &value as *const u32);

    let mut value_mut = 2u32;
    let mut_ptr = ptr::cast_ptr_mut::<u32, u32>(&mut value_mut);
    assert_eq!(mut_ptr, &mut value_mut as *mut u32);

    let mut data = [10u32, 20, 30];
    let data_ptr = data.as_mut_ptr();
    unsafe {
        ptr::write_at(data_ptr, 1, 99);
        let read_value = ptr::read_at(data_ptr as *const u32, 1);
        assert_eq!(read_value, 99);

        ptr::cast_write_at::<u32, u32>(data_ptr, 2, 77);
        let read_value = ptr::cast_read_at::<u32, u32>(data_ptr as *const u32, 2);
        assert_eq!(read_value, 77);
    }
    assert_eq!(data, [10, 99, 77]);
}

#[test]
fn test_slice_get2_mut() {
    let mut data = [1, 2, 3, 4];
    let (left, right) = data.get2_mut(1, 3).expect("indices should be in bounds");
    *left = 20;
    *right = 40;
    assert_eq!(data, [1, 20, 3, 40]);

    assert!(data.get2_mut(2, 2).is_none());
    assert!(data.get2_mut(0, 10).is_none());
}

#[test]
fn test_str_simd_helpers() {
    let ascii = <str as StrExt>::from_ascii_simd(b"hello").expect("valid ascii");
    assert_eq!(ascii, "hello");
    assert!(<str as StrExt>::from_ascii_simd(&[0xff]).is_err());

    let utf8 = <str as StrExt>::from_utf8_simd("hé".as_bytes()).expect("valid utf8");
    assert_eq!(utf8, "hé");
    assert!(<str as StrExt>::from_utf8_simd(&[0xff, 0xff]).is_err());
}

#[cfg(feature = "alloc")]
mod alloc_tests {
    use super::*;
    use stdx::iter::IteratorExt;
    use stdx::string::StringExt;

    #[test]
    fn test_iter_alloc_helpers() {
        let joined = [1, 2, 3].iter().join_("-");
        assert_eq!(joined, "1-2-3");

        let mapped = iter::map_collect_vec([1, 2, 3], |value| value * 3);
        assert_eq!(mapped, vec![3, 6, 9]);

        let filtered =
            iter::filter_map_collect_vec([1, 2, 3, 4], |value| (value % 2 == 1).then_some(value));
        assert_eq!(filtered, vec![1, 3]);
    }

    #[test]
    fn test_string_from_utf8_simd() {
        let value = String::from_utf8_simd(b"hello".to_vec()).expect("valid utf8");
        assert_eq!(value, "hello");

        let err = String::from_utf8_simd(vec![0xff, 0x80]).expect_err("invalid utf8");
        assert_eq!(err.into_bytes(), vec![0xff, 0x80]);
    }
}
