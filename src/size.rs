use std::fmt;
use std::error::Error;

/// Represents a size in bytes.
#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Debug)]
pub struct Size(pub u64);

/// Custom error type for size conversion problems.
#[derive(Debug, Clone)]
pub struct SizeError;

/// Implements Display for user-friendly formatting of SizeError.
impl fmt::Display for SizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid size conversion")
    }
}

/// Implements Error for using SizeError as an error type.
impl Error for SizeError {}

/// Implements functionality for the Size structure.
impl Size {
    /// Constructs a new Size instance from the number of bytes (with error handling).
    pub fn new(bytes: u64) -> Result<Self, SizeError> {
        if bytes <= u64::MAX {
            Ok(Size(bytes))
        } else {
            Err(SizeError)
        }
    }
}

/// Implements Display for user-friendly formatting of the Size structure.
impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut unit = "B";
        let mut size = self.0 as f64;

        let kb  = 1024.0 ;
        let mb = 1024.0 * kb ;
        let gb = 1024.0 * mb ;

        if size < 1024.0 {
            unit = unit;
            size = size;
        }
        else if kb < size && size < mb {
            unit = "KB";
            size = size/kb;
        }
        else if mb <= size && size < gb {
            unit = "MB";
            size = size/mb;
        }
        else if gb <= size {
            unit = "GB";
            size = size/gb;
        }

        write!(f, "{} {:.1}", unit, size)
    }
}

/// Implements the addition operator for the Size structure.
impl std::ops::Add for Size {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Size(self.0 + other.0)
    }
}

/// Implements the conversion from u64 to Size with error handling.
impl TryFrom<u64> for Size {
    type Error = SizeError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value <= u64::MAX {
            Ok(Size(value))
        } else {
            Err(SizeError)
        }
    }
}

/// Module containing tests for the Size structure.
mod tests {
    use super::*;

    /// Tests the creation of a new Size instance.
    #[test]
    fn test_new() {
        let new = Size::new(78564);
        assert!(new.is_ok());
        assert_eq!(new.unwrap().0, 78564);
    }

    /// Tests the addition operation for the Size structure.
    #[test]
    fn test_addition() {
        let a = Size(6666);
        let b = Size(7777);
        let c = a + b;
        assert_eq!(c.0, 14443);
    }

    /// Tests the display for sizes in bytes.
    #[test]
    fn test_display_bytes() {
        assert_eq!(Size(1023).to_string(), "B 1023.0");
    }

    /// Tests the display for sizes in kilobytes.
    #[test]
    fn test_display_kilobytes() {
        assert_eq!(Size(2048).to_string(), "KB 2.0");
    }

    /// Tests the display for sizes in megabytes.
    #[test]
    fn test_display_megabytes() {
        assert_eq!(Size(2 * 1024 * 1024).to_string(), "MB 2.0");
    }

    /// Tests the display for sizes in gigabytes.
    #[test]
    fn test_display_gigabytes() {
        assert_eq!(Size(2 * 1024 * 1024 * 1024).to_string(), "GB 2.0");
    }
}
