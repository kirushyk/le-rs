mod le;

#[cfg(test)]
mod tests {
    use crate::le::tensor::Tensor;
    #[test]
    fn subtensor() {
        let tensor = Tensor::new_2d(&[&[1.0, 2.0, 3.0, 4.0],
                                      &[4.0, 3.0, 2.0, 1.0]]);
    }
}
