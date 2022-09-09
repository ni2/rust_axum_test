pub fn multiply(mut vector: Vec<f64>, factor: f64) -> Vec<f64> {
	vector.iter_mut().for_each(|n| *n *= factor);
	vector
}
