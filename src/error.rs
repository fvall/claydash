#[derive(Debug, Clone)]
pub enum AppError {
    MissingFunction(&'static str),
    InvalidFont(&'static str),
    InvalidDll(&'static str),
    ReloadError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingFunction(fun) => {
                f.write_str("Error: missing function - `")?;
                f.write_str(fun)?;
                f.write_str("`")?;
            }
            Self::InvalidFont(font) => {
                f.write_str("Error: invalid font - ")?;
                f.write_str(font)?;
            }
            Self::InvalidDll(msg) => {
                f.write_str("Error: invalid shared library - ")?;
                f.write_str(msg)?;
            }
            Self::ReloadError(msg) => {
                f.write_str("Error: there was an error when reloading the shared library - ")?;
                f.write_str(msg)?;
            }
        }
        f.write_str("\n")
    }
}

impl std::error::Error for AppError {}
