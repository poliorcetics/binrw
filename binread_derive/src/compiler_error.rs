use proc_macro2::Span;

#[derive(Debug)]
pub enum CompileError {
    SpanError(SpanError),
    Darling(darling::Error)
}

#[derive(Debug)]
pub struct SpanError(pub Span, pub String);

impl SpanError {
    pub fn new<E: Into<String>>(span: Span, err: E) -> Self {
        SpanError(span, err.into())
    }

    pub fn err<E: Into<String>>(span: Span, err: E) -> Result<(), Self> {
        Err(SpanError(span, err.into()))
    }
}

impl From<darling::Error> for CompileError {
    fn from(err: darling::Error) -> Self {
        CompileError::Darling(err)
    }
}

impl From<SpanError> for CompileError {
    fn from(err: SpanError) -> Self {
        CompileError::SpanError(err)
    }
}
