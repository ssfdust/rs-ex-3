#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackType {
    KB,
    SP,
}


impl PackType {
    pub const ALL: [PackType; 2] = [PackType::SP, PackType::KB];
}


impl Default for PackType {
    fn default() -> PackType {
        PackType::SP
    }
}


impl std::fmt::Display for PackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PackType::SP => "SP",
                PackType::KB => "KB",
            }
        )
    }
}
