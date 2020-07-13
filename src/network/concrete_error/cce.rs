use crate::network::error_trait::Error;
use ndarray::ArrayD;

pub struct CategoricalCrossEntropyError {
}

impl CategoricalCrossEntropyError {
  pub fn new() -> Self {
    CategoricalCrossEntropyError{
    }
  }
}



impl Error for CategoricalCrossEntropyError {

  fn get_type(&self) -> String {
    "Binary Crossentropy Error".to_string()
  }

  fn forward(&mut self, input: ArrayD<f32>, _target: ArrayD<f32>) -> ArrayD<f32> {
    input
  }

  fn backward(&mut self, _input: ArrayD<f32>, target: ArrayD<f32>) -> ArrayD<f32>{
    target
  }

  fn loss_from_logits(&mut self, input: ArrayD<f32>, _target: ArrayD<f32>) -> ArrayD<f32> {
    input
  }

  fn deriv_from_logits(&mut self, _input: ArrayD<f32>, target: ArrayD<f32>) -> ArrayD<f32> {
    target 
  }
}


//https://gombru.github.io/2018/05/23/cross_entropy_loss/
//https://towardsdatascience.com/implementing-the-xor-gate-using-backpropagation-in-neural-networks-c1f255b4f20d
//pub fn binary_crossentropy(target: Array1<f32>, output: Array1<f32>) -> Array1<f32> { //should be used after sigmoid
  //assert len of output vector = 1
  //-&target / &output; // + (1.0-target) / (1.0-output)
//}



//https://stats.stackexchange.com/questions/235528/backpropagation-with-softmax-cross-entropy
//pub fn categorical_crossentropy(target: Array1<f32>, output: Array1<f32>) -> Array1<f32> { //should be used after softmax
  //-&target / &output + (1.0-target) / (1.0-output)
  //output - target
//}
