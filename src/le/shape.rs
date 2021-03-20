use std::os::raw::c_void;

#[link(name = "le")]
extern "C" {
    fn le_shape_new_from_data(num_dimensions: usize, sizes: *const usize) -> *const c_void;
}

pub struct Shape {
    pub shape: *const c_void
}

impl Shape {
    pub fn new(num_dimensions: usize, sizes: &[usize]) -> Shape {
        let shape = unsafe { le_shape_new_from_data(num_dimensions, sizes.as_ptr()) };
        Shape { shape }
    }
}
