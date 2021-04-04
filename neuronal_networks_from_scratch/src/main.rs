use ndarray::arr1;
use ndarray::arr2;
mod DenseLayer;

fn simplier_dot() {
    let inputs = arr1(&[1.0, 2.0, 3.0, 2.5]);

    let weights = arr2(&[
        [0.2, 0.8, -0.5, 1.0],
        [0.5, -0.91, 0.26, -0.5],
        [-0.26, -0.27, 0.17, 0.87],
    ]);

    let biases = arr1(&[2.0, 3.0, 0.5]);

    let output = weights.dot(&inputs) + &biases;

    println!("{:?}", output);
}
fn main() {
    let inputs = [1.0, 2.0, 3.0, 2.5];

    let weights = [
        [0.2, 0.8, -0.5, 1.0],
        [0.5, -0.91, 0.26, -0.5],
        [-0.26, -0.27, 0.17, 0.87],
    ];

    let biases = [2.0, 3.0, 0.5];

    let mut layer_outputs = Vec::new(); // Output of current layer

    for (neuron_weights, neuron_bias) in weights.iter().zip(biases.iter()) {
        let mut neuron_output = 0.0; // Output of giiven neuron
        for (n_input, weight) in inputs.iter().zip(neuron_weights.iter()) {
            neuron_output += n_input * weight;
        }
        neuron_output += neuron_bias;
        layer_outputs.push(neuron_output);
    }

    println!("{:?}", layer_outputs);
    simplier_dot();

    let layer_1 = DenseLayer::new(4, 5);
}
