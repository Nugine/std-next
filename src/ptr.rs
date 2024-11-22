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
