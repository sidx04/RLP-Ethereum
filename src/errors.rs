use crate::RLPDecodingError;

impl std::fmt::Display for RLPDecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RLPDecodingError::InvalidData => write!(f, "Data type wasn't expected here!"),
            RLPDecodingError::UnexpectedEOF => write!(f, "Unexpected End of File"),
            RLPDecodingError::UnsupportedType => write!(f, "Unsupported Type"),
        }
    }
}
