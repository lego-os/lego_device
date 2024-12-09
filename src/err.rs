use core::{error::Error, fmt::Display};

/// 设备错误信息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
    DeviceNotFound,
    PermissionDenied,
    DeviceBusy,
    IoError,
    Timeout,
    Disconnected,
    DataCorruption,
    NoSpaceLeft,
    InvalidConfiguration,
    UnsupportedOperation,
    InvalidParameter,
    BadAddress,
    OutOfMemory,
    AlreadyInitialized,
    UnknownError,
}

impl Error for DeviceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DeviceError::DeviceNotFound => Some(&Self::DeviceNotFound),
            DeviceError::PermissionDenied => Some(&Self::PermissionDenied),
            DeviceError::DeviceBusy => Some(&Self::DeviceBusy),
            DeviceError::IoError => Some(&Self::IoError),
            DeviceError::Timeout => Some(&Self::Timeout),
            DeviceError::Disconnected => Some(&Self::Disconnected),
            DeviceError::DataCorruption => Some(&Self::DataCorruption),
            DeviceError::NoSpaceLeft => Some(&Self::NoSpaceLeft),
            DeviceError::InvalidConfiguration => Some(&Self::InvalidConfiguration),
            DeviceError::UnsupportedOperation => Some(&Self::UnsupportedOperation),
            DeviceError::InvalidParameter => Some(&Self::InvalidParameter),
            DeviceError::BadAddress => Some(&Self::BadAddress),
            DeviceError::OutOfMemory => Some(&Self::OutOfMemory),
            DeviceError::AlreadyInitialized => Some(&Self::AlreadyInitialized),
            DeviceError::UnknownError => Some(&Self::UnknownError),
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    fn provide<'a>(&'a self, request: &mut core::error::Request<'a>) {
        request.provide_value(*self);
    }
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DeviceError::DeviceNotFound => write!(f, "Device Not Found"),
            DeviceError::PermissionDenied => write!(f, "Device permission deny"),
            DeviceError::DeviceBusy => write!(f, "Device busy"),
            DeviceError::IoError => write!(f, "Device Input/Output error"),
            DeviceError::Timeout => write!(f, "Device operation timeout"),
            DeviceError::Disconnected => write!(f, "Device disconnected"),
            DeviceError::DataCorruption => write!(f, "Device data corruption"),
            DeviceError::NoSpaceLeft => write!(f, "Device have no space"),
            DeviceError::InvalidConfiguration => write!(f, "Device configuration invalid"),
            DeviceError::UnsupportedOperation => write!(f, "Device not support operation"),
            DeviceError::InvalidParameter => write!(f, "Device parameter invalid"),
            DeviceError::BadAddress => write!(f, "Device address illegal"),
            DeviceError::OutOfMemory => write!(f, "Device out of memory"),
            DeviceError::AlreadyInitialized => write!(f, "Device already initialized"),
            DeviceError::UnknownError => write!(f, "Device unknown error"),
        }
    }
}
