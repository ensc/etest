#[derive(Debug, Copy, Clone)]
pub enum Error {
    NoValue,
    BadValue,
    BadInteger,
    BadType,

    FunctionDeclIncomplete,
    FunctionDeclBad,
    FunctionNoBody,
    FunctionExtraTokens,
}
