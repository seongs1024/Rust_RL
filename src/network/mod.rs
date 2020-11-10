/// This submodule provides various error function.
///
/// Beside of the normal forward() and backward() functions some Error functions implement both functions in a *_from_logits() variant.  
/// They merge their own implementation with the previous activation function in a numerically more stable way.  
/// Better known examples are Softmax+CategoricalCrossEntropy or Sigmoid+BinaryCrossEntropy.  
/// When used as part of nn, the appropriate functions are automatically picked.  
pub mod error;

/// This submodule offers multiple layer implementation.
///
/// The forward and backward functions have to accept and return data in the form ArrayD\<f32>.  
/// Common activation functions are bundled under activation_layer.  
pub mod layer;

/// This submodules bundles all neural network related functionalities.
///
/// A new neural network is created with new1d(..), new2d(..), or new3d(..).  
/// For higher-dimensional input a new() function is available which accepts arbitrary sized input.  
/// The input shape, error function and the optimizer are set during network creation.  
/// If an individual error function or optimizer should be used, they can be set (overwriting the former one) by using set_error_function() or set_optimizer().  
/// Default layers can be added using convenience functions like add_dense(..) or add_convolution(..) which allow setting the main parameters.  
/// For a higher level of controll, or to add own layers, the store_layer(Box<dyn Layer>) function can be used to add a layer to the current network.  
pub mod nn;

/// This submodule offers 5 of the most common optimizers.
///
/// noop falls back to the default sgd.
pub mod optimizer;

mod tests;
