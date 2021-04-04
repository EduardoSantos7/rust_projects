use ndarray::Array;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

pub struct DenseLayer {
    weights: ndarray::ArrayBase,
    biases: ndarray::ArrayBase,
    output: Vec<Vec<f64>>,
}

impl DenseLayer {
    fn new(self, n_inputs: i32, n_neurons: i32) {
        self.weights = Array::random((n_inputs, n_neurons), Uniform::new(0., 0.1)).into_dyn();
        self.biases = Array::zeros((1, n_inputs));
    }

    fn forward(self, inputs: Array) {
        self.output = inputs.dot(&self.weights) + self.biases;
    }
}
