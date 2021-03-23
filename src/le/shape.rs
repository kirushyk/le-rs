use std::os::raw::c_void;

#[link(name = "le")]
extern "C" {
    fn le_shape_new_uninitialized(num_dimensions: usize) -> *mut c_void;
    fn le_shape_set_size(c_shape: *mut c_void, dimension: usize, size: u32);
    fn le_shape_get_size(c_shape: *const c_void, dimension: usize) -> u32;
}

pub struct Shape {
    pub c_shape: *const c_void
}

impl Shape {
    pub fn new(num_dimensions: usize, sizes: &[u32]) -> Shape {
        let c_shape = unsafe { le_shape_new_uninitialized(num_dimensions) };
        for (index, size) in sizes.iter().enumerate() {
            unsafe {
                le_shape_set_size(c_shape, index, *size);
            }
        }
        Shape { c_shape }
    }
    /// @todo: impl std::ops::Index
    fn dim(&self, index: usize) -> Option<u32> {
        unsafe {
            #[repr(C)]
            struct CShape {
                num_dimensions: usize,
                sizes: *mut u32
            }
            if index < (*(self.c_shape as *const CShape)).num_dimensions {
                Some(le_shape_get_size(self.c_shape, index))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shape() {
        let shape = Shape::new(4, &[5, 6, 7, 8]);
        assert_eq!(shape.dim(0), Some(5));
        assert_eq!(shape.dim(1), Some(6));
        assert_eq!(shape.dim(2), Some(7));
        assert_eq!(shape.dim(3), Some(8));
        assert_eq!(shape.dim(4), None);
    }
}
