use std::alloc::{alloc_zeroed, Layout};

pub fn alloc_box_buffer<T: Sized>(len: usize) -> Box<[T]> {
    if len == 0 {
        return <Box<[T]>>::default();
    }
    let layout = Layout::array::<T>(len).unwrap();
    let ptr = unsafe { alloc_zeroed(layout) as *mut T };
    let slice_ptr = core::ptr::slice_from_raw_parts_mut(ptr, len);
    unsafe { Box::from_raw(slice_ptr) }
}