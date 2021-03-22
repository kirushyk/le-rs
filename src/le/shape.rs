use std::os::raw::c_void;

#[link(name = "le")]
extern "C" {
    fn le_shape_new_uninitialized(num_dimensions: usize) -> *mut c_void;
    fn le_shape_set_size(shape: *mut c_void, dimension: usize, size: u32);
}

pub struct Shape {
    pub shape: *const c_void
}

impl Shape {
    pub fn new(num_dimensions: usize, sizes: &[u32]) -> Shape {
        let shape = unsafe { le_shape_new_uninitialized(num_dimensions) };
        for (index, size) in sizes.iter().enumerate() {
            unsafe {
                le_shape_set_size(shape, index, *size);
            }
        }
        Shape { shape }
    }
}
