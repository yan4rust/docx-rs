use strong_xml::{XmlRead, XmlWrite};

/// Numbering Id
///
/// ```rust
/// use docx::formatting::*;
///
/// let id = NumberingId::from(42usize);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:cols")]
pub struct PageCols {
    #[xml(attr = "w:space")]
    pub space: Option<usize>,
}

// impl<T: Into<usize>> From<T> for NumberingId {
//     fn from(val: T) -> Self {
//         NumberingId { value: val.into() }
//     }
// }

// __xml_test_suites!(
//     NumberingId,
//     NumberingId::from(40usize),
//     r#"<w:numId w:val="40"/>"#,
// );
