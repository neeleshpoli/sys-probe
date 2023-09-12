#[cfg(target_os = "windows")]
pub mod windows_error{
    use std::{error::Error, fmt, alloc::LayoutError, env::VarError};

    use windows::core::Error as WindowsError;

    pub type SysProbeResult<T> = Result<T, SysProbeError>;

    #[derive(Debug)]
    pub enum SysProbeError {
        WindowsAPIError(WindowsError),
        DateTimeParsingError(),
        LayoutError(LayoutError),
        EnvironmentVariableError(VarError),
    }

    impl Error for SysProbeError {}

    impl fmt::Display for SysProbeError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl From<windows::core::Error> for SysProbeError {
        fn from(error: windows::core::Error) -> Self {
            SysProbeError::WindowsAPIError(error)
        }
    }

    impl From<LayoutError> for SysProbeError {
        fn from(error: LayoutError) -> Self {
            SysProbeError::LayoutError(error)
        }
    }

    impl From<VarError> for SysProbeError {
        fn from(error: VarError) -> Self {
            SysProbeError::EnvironmentVariableError(error)
        }
    }
}