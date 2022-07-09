use strong_xml::{XmlRead, XmlWrite};

/// WidowControl
///
/// ```rust
/// use docx::formatting::*;
///
/// let WidowControl = WidowControl::from(false);
/// let WidowControl = WidowControl::from(true);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:unhideWhenUsed")]
pub struct UnhideWhenUsed {
    #[xml(attr = "w:val")]
    pub value: Option<usize>,
}
