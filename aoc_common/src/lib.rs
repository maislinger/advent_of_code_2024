pub type AocResult = Result<String, AocError>;

#[derive(Debug)]
pub enum AocError {
    InvalidInput,
    ParseIntError(std::num::ParseIntError),
}

impl std::fmt::Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AocError::InvalidInput => write!(f, "Invalid input"),
            AocError::ParseIntError(p) => write!(f, "ParseIntError: {}", p),
        }
    }
}

impl std::error::Error for AocError {}

impl From<std::num::ParseIntError> for AocError {
    fn from(e: std::num::ParseIntError) -> Self {
        AocError::ParseIntError(e)
    }
}

pub fn split_to_array<'a, const N: usize>(line: &'a str, pat: &str) -> Option<[&'a str; N]> {
    let mut result = [""; N];
    let mut splitted = line.split(pat);

    for r in result.iter_mut().take(N) {
        *r = splitted.next()?;
    }

    Some(result)
}

pub fn split_whitespace_to_array<const N: usize>(line: &str) -> Option<[&str; N]> {
    let mut result = [""; N];
    let mut splitted = line.split_whitespace();

    for r in result.iter_mut().take(N) {
        *r = splitted.next()?;
    }

    Some(result)
}
