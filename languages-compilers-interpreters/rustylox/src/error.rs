#[derive(Debug)]
pub enum ErrorKind {
    ScanError,
    ReadFileError,
    ParseError,
    EvaluatorError,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_kind = match self {
            ErrorKind::ScanError => "ScanError",
            ErrorKind::ReadFileError => "ReadFileError",
            ErrorKind::ParseError => "ParseError",
            ErrorKind::EvaluatorError => "EvaluatorError",
        };
        write!(f, "Error: {}", error_kind)
    }
}

struct Error {
    line: u32,
    start: u32,
    end: u32,
    message: String,
    kind: ErrorKind,
}

pub fn error(line: u32, start: u32, end: u32, message: String, kind: ErrorKind) {
    report(Error {
        line,
        start,
        end,
        message,
        kind,
    });
}

fn report(err: Error) {
    eprintln!(
        "{}:{}-{} {}: {}",
        err.line, err.start, err.end, err.kind, err.message
    );
}
