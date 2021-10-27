use thiserror::Error;

/// WordCountError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to compile WGSL shader")]
    WgslCompilationFailure,

    #[error("Unsupported shader type")]
    UnsupportedShaderType,

    #[error("No shader source supplied")]
    NoShaderSource,

    #[error("No device supplied")]
    NoDeviceSupplied,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

