/// A  matrix or submatrix that can be explicitly accessed or manipulated.  
/// 
/// This is good for matrices that have been loaded into memory for
/// manipulation.  It is bad for matrices that are being streamed.  
pub trait Matrix {
    
    /// Constrain the matrix to a range of indeces at the specified depth.
    /// 
    /// The `depth` refers to a nesting depth within the matrix. `start` and
    /// `end` refer to the start and the end of the specified range. Indeces
    /// start at 0 and `end` is non-inclusive.
    fn slice(&self, depth: uint, start: uint, end: uint) -> Option<Self>;

    /// Calculate the shape of this matrix
    /// 
    /// Returns a list of numbers, each one corresponding to the number of
    /// entries in a given dimension of the matrix
    fn shape(&self) -> Vec<uint>;

    fn reshape(&mut self, shape: Vec<uint>) -> Result<(), &'static str>;
}

// TODO: implement a DeepVecMatrix for cases where we want to trade time for
//       space on matrices with inconsistent axis dimensions.

/// A matrix that stores its data as a flat vector of strings
/// 
/// This is good for matrices that need to be reshaped, but bad for matrices
/// that have inconsistent axis dimensions (e.g. one column with 100 entries
/// and another column with 2).
struct FlatVecMatrix {
    dat: Vec<&str>,
    dat_shape: Vec<uint>,
}

impl FlatVecMatrix {
    /// Construct a new Matrix from a flat vector and a shape
    fn from_vec(dat: &Vec<&str>, shape: &Vec<uint>) -> Result<FlatVecMatrix, 
            &'static str> {
        let out = FlatVecMatrix { dat: dat.clone(), shape: shape.clone() };
        let valid = out.valid_shape();
        match valid {
            Err =>  valid,
            Ok  =>  Ok(out)
        }
    }

    fn valid_shape(&self) -> Result<(), &'static str> {
        let count = shape.iter().fold(1, |a, &b| a * b);
        if count != dat.len() {
            Err("Shape does not match input length")
        }
        Ok()
    }
}


impl Matrix for FlatVecMatrix {

}
