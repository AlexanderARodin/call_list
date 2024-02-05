use std::error::Error;

pub type ResultOf< T > = Result< T, Box<dyn Error> >;


