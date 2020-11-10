#[cfg(test)]
#[allow(non_snake_case)]
mod MLP {
    use crate::network::nn::NeuralNetwork;
    use datasets::mnist;
    use ndarray::{array, Array2};
    use rand::Rng;

    fn new(i_dim: usize, bs: usize, lr: f32) -> NeuralNetwork {
        let mut nn = NeuralNetwork::new1d(i_dim, "bce".to_string(), "adam".to_string());
        nn.set_batch_size(bs);
        nn.set_learning_rate(lr);
        nn.add_dense(2); //Dense with 2 output neurons
        nn.add_activation("sigmoid"); //Sigmoid
        nn.add_dense(1); //Dense with 1 output neuron
        nn.add_activation("sigmoid"); //Sigmoid
        nn
    }

    fn test(mut nn: NeuralNetwork, input: Array2<f32>, feedback: Array2<f32>, testname: String) {
        let mut current_input;
        let mut current_feedback;
        for i in 0..input.nrows() {
            current_input = input.row(i).into_owned().clone();
            current_feedback = feedback.row(i).into_owned().clone();

            let prediction = nn.predict1d(current_input.clone());
            //let diff = nn.loss_from_prediction(prediction.clone(), current_feedback.clone());
            let diff =
                nn.loss_from_input(current_input.clone().into_dyn(), current_feedback.clone());
            assert!(
                diff < 0.2,
                "failed learning: {}. Achieved loss: {}\n input: {} output was: {:} should {:}",
                testname,
                diff,
                current_input.clone(),
                prediction,
                current_feedback
            );
        }
    }

    fn train(nn: &mut NeuralNetwork, num: usize, input: &Array2<f32>, feedback: &Array2<f32>) {
        let mut pos;
        let mut current_input;
        let mut current_feedback;
        for _ in 0..num {
            pos = rand::thread_rng().gen_range(0, input.nrows()) as usize;
            current_input = input.row(pos).into_owned().clone();
            current_feedback = feedback.row(pos).into_owned().clone();
            nn.train1d(current_input, current_feedback);
        }
    }

    #[allow(non_snake_case)]
    #[test]
    #[ignore]
    fn MNIST() {
        let (trn_size, rows, cols) = (60_000, 28, 28);

        // Deconstruct the returned Mnist struct.
        let mnist::Data { trn_img, .. } = mnist::new_normalized();
        assert_eq!(trn_img.shape(), &[trn_size, rows, cols]);
    }

    #[test]
    fn and() {
        let input = array![[0., 0.], [0., 1.], [1., 0.], [1., 1.], [1., 1.], [1., 1.]]; // AND
        let feedback = array![[0.], [0.], [0.], [1.], [1.], [1.]]; //AND work ok with 200k examples (10 and 01 are classified correctly, but close to 0.5)
        let mut nn = new(2, 6, 0.1);
        train(&mut nn, 1_000, &input, &feedback);
        test(nn, input, feedback, "and".to_string());
    }

    #[test]
    fn or() {
        let input = array![[0., 0.], [0., 0.], [0., 0.], [0., 1.], [1., 0.], [1., 1.]]; // OR
        let feedback = array![[0.], [0.], [0.], [1.], [1.], [1.]]; //OR works great with 200k examples
        let mut nn = new(2, 6, 0.1);
        train(&mut nn, 5000, &input, &feedback);
        test(nn, input, feedback, "or".to_string());
    }

    #[test]
    fn not() {
        let input = array![[0.], [1.]];
        let feedback = array![[1.], [0.]]; // NOT works great with 200k examples
        let mut nn = new(1, 1, 0.1);
        train(&mut nn, 500, &input, &feedback);
        test(nn, input, feedback, "not".to_string());
    }

    #[test]
    fn first() {
        let input = array![[0., 0.], [0., 1.], [1., 0.], [1., 1.]]; // FIRST
        let feedback = array![[0.], [0.], [1.], [1.]]; //First works good with 200k examples
        let mut nn = new(2, 4, 0.1);
        train(&mut nn, 500, &input, &feedback);
        test(nn, input, feedback, "first".to_string());
    }

    #[test]
    #[ignore]
    fn xor() {
        let input = array![[0., 0.], [0., 1.], [1., 0.], [1., 1.]];
        let feedback = array![[0.], [1.], [1.], [0.]]; //XOR
        let mut nn = new(2, 4, 0.1);
        train(&mut nn, 30_000, &input, &feedback);
        test(nn, input, feedback, "xor".to_string());
    }
}
