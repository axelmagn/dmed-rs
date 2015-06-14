//! This package manages delimiters for data matrices
//! 

#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

use regex::Regex;

/// Delimiters specify where to split up a data matrix
struct Delimiters {
    row_delim: u8,
    patterns: Vec<Regex>,
}

impl Delimiters {
    pub fn new(delim_str: &str) -> Delimiters {
        let mut patterns = Vec::new();
        Delimiters { 
            row_delim: 0xA,
            patterns: patterns,
        }
    }
}




