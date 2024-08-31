#[derive(Debug, Copy, Clone)]
pub enum Error {
    NoValue,
    BadValue,
    BadInteger,
    BadType,
    ExtraData,

    FunctionDeclIncomplete,
    FunctionDeclBad,
    FunctionNoBody,
    FunctionExtraTokens,
}
