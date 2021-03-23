
use le_rs::le::tensor::Tensor;

fn main() {
    let _tensor = Tensor::new_2d(&[&[1.0, 2.0, 3.0, 4.0],
                                   &[4.0, 3.0, 2.0, 1.0]]);
}
