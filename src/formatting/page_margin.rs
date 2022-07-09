use strong_xml::{XmlRead, XmlWrite};

/// Numbering Id
///
/// ```rust
/// use docx_rust::formatting::*;
///
/// let id = NumberingId::from(42usize);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:pgMar")]
pub struct PageMargin {
    #[xml(attr = "w:top")]
    pub top: Option<usize>,
    #[xml(attr = "w:right")]
    pub right: Option<usize>,
    #[xml(attr = "w:bottom")]
    pub bottom: Option<usize>,
    #[xml(attr = "w:left")]
    pub left: Option<usize>,
    #[xml(attr = "w:header")]
    pub header: Option<usize>,
    #[xml(attr = "w:footer")]
    pub footer: Option<usize>,
    #[xml(attr = "w:gutter")]
    pub gutter: Option<usize>,
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
