use strong_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

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
#[xml(tag = "w:uiPriority")]
pub struct Priority {
    #[xml(attr = "w:val")]
    pub value: Option<usize>,
}
