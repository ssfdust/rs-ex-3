#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreName {
    KB,
    SP,
}


impl PreName {
    pub const ALL: [PreName; 2] = [PreName::SP, PreName::KB];
}


impl Default for PreName {
    fn default() -> PreName {
        PreName::SP
    }
}


impl std::fmt::Display for PreName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PreName::SP => "SP",
                PreName::KB => "KB",
            }
        )
    }
}
