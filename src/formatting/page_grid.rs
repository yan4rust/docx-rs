use std::str::FromStr;

use strong_xml::{XmlRead, XmlWrite};

use crate::{__string_enum, __xml_test_suites, __setter};

/// Numbering Id
///
/// ```rust
/// use docx_rust::formatting::*;
///
/// let id = NumberingId::from(42isize);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:docGrid")]
pub struct PageGrid {
    #[xml(attr = "w:type")]
    pub ty: Option<GridType>,
    #[xml(attr = "w:linePitch")]
    pub line_pitch: Option<isize>,
    #[xml(attr = "w:charSpace")]
    pub char_space: Option<isize>,
}

impl PageGrid {
    __setter!(ty: Option<GridType>);
    __setter!(line_pitch: Option<isize>);
    __setter!(char_space: Option<isize>);
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub enum GridType {
    #[default]
    Default,       //	No Document Grid
    Lines,         //	Line Grid Only
    LinesAndChars, //	Line and Character Grid
    SnapToChars,   //	Character Grid Only
}

// hack
impl FromStr for GridType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let v =match  value {
            "0" | "default" => Self::Default,
            "1" | "lines" => Self::Lines,
            "2" | "linesAndChars" => Self::LinesAndChars,
            "3" | "snapToChars" => Self::SnapToChars,
            _ => Self::Default
        };

        Ok(v)
    }
}

impl std::fmt::Display for GridType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match  *self {
            GridType::Default => write!(f, "default"),
            GridType::Lines => write!(f, "lines"),
            GridType::LinesAndChars => write!(f, "lineAndChars"),
            GridType::SnapToChars => write!(f, "snapToChars"),
        }
    }
}

// __string_enum! {
//     GridType {
//         Default = "default",
//         Lines = "lines",
//         LinesAndChars = "linesAndChars",
//         SnapToChars = "snapToChars",
//     }
// }

// impl<T: Into<isize>> From<T> for NumberingId {
//     fn from(val: T) -> Self {
//         NumberingId { value: val.into() }
//     }
// }

__xml_test_suites!(
    PageGrid,
    PageGrid::default().ty(GridType::Lines),
    r#"<w:docGrid w:type="lines"/>"#,
);
