use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, Default, Copy, Clone, Eq, Hash, PartialEq, EnumString)]
pub enum AssetName {
    #[default]
    A,
    B,
    C,
    D,
    Unknown,
}

impl AssetName {
    pub fn next(&self) -> Self {
        use self::AssetName::*;
        match *self {
            A => B,
            B => C,
            C => D,
            D => Unknown,
            Unknown => Unknown,
        }
    }

    pub fn index(&self) -> usize {
        *self as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_from_str_works() {
        let name_variant = AssetName::from_str("A").unwrap();

        assert_eq!(AssetName::A, name_variant);
    }

    #[test]
    fn next_value_works() {
        let a = AssetName::A;
        let b = AssetName::B;
        let d = AssetName::D;
        let unknown = AssetName::Unknown;

        assert_eq!(a.next(), AssetName::B);
        assert_eq!(b.next(), AssetName::C);
        assert_eq!(d.next(), AssetName::Unknown);
        assert_eq!(unknown.next(), AssetName::Unknown);
    }
}
