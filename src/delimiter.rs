/// Delims describes a set of delimiters used to parse a string into a matrix.
pub trait Delims {
    /// Create a DelimSet
    fn from_string(delim: &str) -> Option<Self>;

    // TODO: DelimSet member functions
}

// TODO: delimiter implementation
