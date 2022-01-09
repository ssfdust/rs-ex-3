#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirstName {
    LAS,
    BVT,
    BDSEC,
    SAS,
    CSV,
    BDLOG,
    NFA,
}

impl FirstName {
    pub const ALL: [FirstName; 7] = [
        FirstName::LAS,
        FirstName::BVT,
        FirstName::BDSEC,
        FirstName::SAS,
        FirstName::CSV,
        FirstName::BDLOG,
        FirstName::NFA,
    ];
}

impl Default for FirstName {
    fn default() -> FirstName {
        FirstName::LAS
    }
}

impl std::fmt::Display for FirstName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FirstName::LAS => "LAS",
                FirstName::BVT => "BVT",
                FirstName::BDSEC => "BDSEC",
                FirstName::SAS => "SAS",
                FirstName::CSV => "CSV",
                FirstName::BDLOG => "BDLOG",
                FirstName::NFA => "NFA",
            }
        )
    }
}
