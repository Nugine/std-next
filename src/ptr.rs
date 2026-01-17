#![allow(unsafe_code, clippy::missing_safety_doc)]

#[inline(always)]
pub fn cast_ptr<T: ?Sized, U>(p: &T) -> *const U {
    <*const T>::cast(p)
}

#[inline(always)]
pub fn cast_ptr_mut<T: ?Sized, U>(p: &mut T) -> *mut U {
    <*mut T>::cast(p)
}

#[inline(always)]
pub unsafe fn read_at<T>(src: *const T, idx: usize) -> T {
    src.add(idx).read()
}

#[inline(always)]
pub unsafe fn write_at<T>(dst: *mut T, idx: usize, val: T) {
    dst.add(idx).write(val);
}

#[inline(always)]
pub unsafe fn cast_read_at<T, U>(src: *const T, idx: usize) -> U {
    src.add(idx).cast::<U>().read()
}

#[inline(always)]
pub unsafe fn cast_write_at<T, U>(dst: *mut T, idx: usize, val: U) {
    dst.add(idx).cast::<U>().write(val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unsafe_code)]
    #[test]
    fn test_ptr_helpers() {
        let value = 1u32;
        let const_ptr = cast_ptr::<u32, u32>(&value);
        assert_eq!(const_ptr, &value as *const u32);

        let mut value_mut = 2u32;
        let mut_ptr = cast_ptr_mut::<u32, u32>(&mut value_mut);
        assert_eq!(mut_ptr, &mut value_mut as *mut u32);

        let mut data = [10u32, 20, 30];
        let data_ptr = data.as_mut_ptr();
        unsafe {
            write_at(data_ptr, 1, 99);
            let read_value = read_at(data_ptr as *const u32, 1);
            assert_eq!(read_value, 99);

            cast_write_at::<u32, u32>(data_ptr, 2, 77);
            let read_value = cast_read_at::<u32, u32>(data_ptr as *const u32, 2);
            assert_eq!(read_value, 77);
        }
        assert_eq!(data, [10, 99, 77]);
    }
}
