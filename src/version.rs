use std::fmt;

/// Represents a version of the HTTP spec.
#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Version(Icap);

impl Version {
    pub const ICAP_10: Version = Version(Icap::Icap10);
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
enum Icap {
    Icap10,
}

impl Default for Version {
    #[inline]
    fn default() -> Version {
        Version::ICAP_10
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            Icap::Icap10 => "ICAP/1.0",
        })
    }
}
