pub unsafe fn as_buf<T: Sized>(data: &T) -> &[u8] {
    core::slice::from_raw_parts(
        data as *const T as *const u8,
        std::mem::size_of::<T>()
    )
}