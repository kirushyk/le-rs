use std::os::raw::c_void;
use crate::le::shape::Shape;

#[link(name = "le")]
extern "C" {
    fn le_tensor_new_from_data(element_type: usize, shape: *const c_void, data: *const c_void) -> *mut c_void;
}

pub struct Tensor {
    tensor: *mut c_void
}

impl Tensor {
    pub fn new() -> Tensor {
        Tensor { tensor: std::ptr::null_mut() }
    }
    pub fn new_2d(data: &[&[f32]]) -> Tensor {
        let shape = Shape::new(2, &[1, 1]);
        let tensor = unsafe { le_tensor_new_from_data(1, shape.shape, std::ptr::null()) };
        Tensor { tensor }
    }
}

impl Drop for Tensor {
    fn drop(&mut self) {

    }
}
