use std::os::raw::c_void;
use std::ops::AddAssign;
use crate::le::shape::Shape;

#[repr(C)]
pub enum Type {
    VOID,
    INT8,
    UINT8,
    INT16,
    UINT16,
    INT32,
    UINT32,
    FLOAT16,
    FLOAT32,
    FLOAT64
}

#[link(name = "le")]
extern "C" {
    fn le_tensor_new_uninitialized(element_type: Type, c_shape: *const c_void) -> *mut c_void;
    fn le_tensor_get_data(c_tensor: *mut c_void) -> *mut c_void;
    fn le_tensor_free(c_tensor: *mut c_void);
    fn le_tensor_add_tensor(c_tensor: *mut c_void, rhs: *const c_void);
}

pub struct Tensor {
    c_tensor: *mut c_void
}

impl Tensor {
    pub fn new_2d(data: &[&[f32]]) -> Tensor {
        let rows = data.len();
        let mut columns = 0;
        for row in data.iter() {
            if row.len() > columns {
                columns = row.len();
            }
        }
        let shape = Shape::new(2, &[rows as u32, columns as u32]);
        let c_tensor = unsafe { le_tensor_new_uninitialized(Type::FLOAT32, shape.c_shape) };
        let raw_data = unsafe { le_tensor_get_data(c_tensor) as *mut f32 };
        for (row_index, row) in data.iter().enumerate() {
            unsafe {
                let dst_ptr = raw_data.offset(row_index as isize);
                std::ptr::copy_nonoverlapping(row.as_ptr(), dst_ptr, columns);
            }
        }
        Tensor { c_tensor }
    }
}

impl AddAssign for Tensor {
    fn add_assign(&mut self, rhs: Tensor) {
        unsafe {
            le_tensor_add_tensor(self.c_tensor, rhs.c_tensor);
        }
    }
}

impl Drop for Tensor {
    fn drop(&mut self) {
        unsafe {
            le_tensor_free(self.c_tensor);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum() {
        let mut a = Tensor::new_2d(&[&[5.0, 6.0, 7.0, 8.0]]);
        let b = Tensor::new_2d(&[&[1.0, 2.0, 3.0, 4.0]]);
        a += b;
    }
}
