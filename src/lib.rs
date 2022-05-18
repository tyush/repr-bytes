#![deny(missing_docs)]
//! Small, simple library to convert byte amounts to
//! pretty, human readable sizes.
//!
//! # Quickstart
//! ```rust
//! # use repr_size::*;
//! let my_file_size = Size::from(54222);
//!
//! println!("{}", my_file_size); // "54.2 KB"
//! println!("{}", my_file_size.to_si_string()); // "53.0 KiB"
//! println!("{}", my_file_size.repr(Units::Bytes)); // "54222 B"
//! ```
//!
//! # Features
//! `serde` - enables serialization/deserialization of `Size` <-> usize

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
/// Different units available for representing a Size.
///
/// # Usage
/// ```rust
/// # use repr_size::{Size, Units};
/// let my_file_size = Size::from(2300);
/// println!("{}", my_file_size); // 2.3 kB
/// println!("{}", my_file_size.to_string()); // 2.3 kB
/// println!("{}", my_file_size.to_si_string()); // 2.2 KiB
/// println!("{}", my_file_size.repr(Units::Bytes)); // 2300 B
///
/// let twenty_three_kilobytes = Size::from_units(23, Units::Kilobytes);
/// ```
pub enum Units {
    /// Base unit. Equal to 1 byte, or 8 bits, or sizeof(u8), or 1/1000th of a Kilobyte.
    Bytes,

    /// (kB) 1000 bytes.
    Kilobytes,
    /// (KiB) 1024 bytes.
    Kibibytes,

    /// (MB) 1000^2 bytes.
    Megabytes,
    /// (MiB) 1024^2 bytes
    Mebibytes,

    /// (GB) 1000^3 bytes.
    Gigabytes,
    /// (GiB) 1024^3 bytes.
    Gibibytes,

    /// (TB) 1000^4 bytes.
    Terabytes,
    /// (TiB) 1024^4 bytes.
    Tebibytes,

    /// (PB) 1000^5 bytes.
    Petabytes,
    /// (PiB) 1024^5 bytes.
    Pebibytes,
}

impl Units {
    /// Returns the amount of bytes this type represents, ie Units::Kilobytes == 1024
    pub fn bytes(&self) -> usize {
        match self {
            Self::Bytes => 1,
            Self::Kilobytes => 1000,
            Self::Kibibytes => 1024,
            Self::Megabytes => 1000 ^ 2,
            Self::Mebibytes => 1024 ^ 2,
            Self::Gigabytes => 1000 ^ 3,
            Self::Gibibytes => 1024 ^ 3,
            Self::Terabytes => 1000 ^ 4,
            Self::Tebibytes => 1024 ^ 4,
            Self::Petabytes => 1000 ^ 5,
            Self::Pebibytes => 1024 ^ 5,
        }
    }
}

impl Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bytes => "B",
                Self::Kilobytes => "kB",
                Self::Kibibytes => "KiB",
                Self::Megabytes => "MB",
                Self::Mebibytes => "MiB",
                Self::Gigabytes => "GB",
                Self::Gibibytes => "GiB",
                Self::Terabytes => "TB",
                Self::Tebibytes => "TiB",
                Self::Petabytes => "PB",
                Self::Pebibytes => "PiB",
            }
        )
    }
}

/// Represents an amount of bytes.
/// Create these by using `Size::from(usize)` or `Size::from_units(usize, Units)`
#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "usize", into = "usize"))]
pub struct Size(usize);

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Size {
    /// Gets the largest non-SI unit that can represent this number without all significant digits being in the decimal.
    ///
    /// # Usage
    /// ```rust
    /// # use repr_size::*;
    /// let just_over_one_kilobyte = Size::from(1001);
    /// let just_under_one_kilobyte = Size::from(999);
    ///
    /// assert_eq!(just_over_one_kilobyte.get_units(), Units::Kilobytes);
    /// assert_eq!(just_under_one_kilobyte.get_units(), Units::Bytes);
    /// ```
    pub fn get_units(&self) -> Units {
        match self.0 {
            x if x < 1000usize => Units::Bytes,
            x if x < 1000usize.pow(2) => Units::Kilobytes,
            x if x < 1000usize.pow(3) => Units::Megabytes,
            x if x < 1000usize.pow(4) => Units::Gigabytes,
            x if x < 1000usize.pow(5) => Units::Terabytes,
            _ => Units::Petabytes,
        }
    }

    /// Gets the largest SI unit that can represent this number without all significant digits being in the decimal.
    ///
    /// # Usage
    /// ```rust
    /// # use repr_size::*;
    /// let just_over_one_kibibyte = Size::from(1025);
    /// let just_under_one_kibibyte = Size::from(1022);
    ///
    /// assert_eq!(just_over_one_kibibyte.get_si_units(), Units::Kibibytes);
    /// assert_eq!(just_under_one_kibibyte.get_si_units(), Units::Bytes);
    /// ```
    pub fn get_si_units(&self) -> Units {
        match self.0 {
            x if x < 1024usize => Units::Bytes,
            x if x < 1024usize.pow(2) => Units::Kibibytes,
            x if x < 1024usize.pow(3) => Units::Mebibytes,
            x if x < 1024usize.pow(4) => Units::Gibibytes,
            x if x < 1024usize.pow(5) => Units::Tebibytes,
            _ => Units::Pebibytes,
        }
    }

    /// Returns the size represented as an amount and a non-SI unit.
    pub fn to_string(&self) -> String {
        let unit = self.get_units();
        let number = self.0 as f32 / unit.bytes() as f32;
        format!("{:.1} {}", number, unit)
    }

    /// Returns the size represented as an amount and a unit.
    pub fn to_si_string(&self) -> String {
        let unit = self.get_si_units();
        let number = self.0 as f32 / unit.bytes() as f32;
        format!("{:.1} {}", number, unit)
    }

    /// Returns a string representation of the size using
    /// the given unit of bytes.
    /// ```rust
    /// # use repr_size::*;
    /// let twenty_two_kb = Size::from(22000);
    ///
    /// println!("{}", twenty_two_kb.repr(Units::Bytes)); // "22000 B"
    /// println!("{}", twenty_two_kb.repr(Units::Kibibytes)); // "21.4 KiB"
    /// ```
    pub fn repr(&self, unit: Units) -> String {
        let number = self.0 as f32 / unit.bytes() as f32;
        format!("{:.1} {}", number, unit)
    }

    /// Returns a Size derived from unit's byte amount times the number given.
    pub fn from_units(x: usize, unit: Units) -> Size {
        Self(x * unit.bytes())
    }
}

impl From<usize> for Size {
    fn from(rhs: usize) -> Self {
        Self(rhs)
    }
}

impl Into<usize> for Size {
    fn into(self) -> usize {
        self.0
    }
}

impl TryFrom<isize> for Size {
    type Error = ();

    /// Will error if x < 0.
    fn try_from(rhs: isize) -> Result<Self, ()> {
        Ok(Self(rhs.try_into().map_err(|_| ())?))
    }
}
