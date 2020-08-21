/// The structure that is returned if the encode or decode functions failed.
#[derive(Debug)]
pub struct TranslationError {
    /// Vec of all unsupported characters causing the error.
    pub unsupported_characters: Vec<String>,
    /// The completed parse result. Failed characters have been replaced by `#`
    pub result: String,
}

pub mod decode;pub mod encode;
