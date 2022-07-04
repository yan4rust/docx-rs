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
#[xml(tag = "w:widowControl")]
pub struct WidowControl {
    #[xml(attr = "w:val")]
    pub value: Option<usize>,
}

impl From<bool> for WidowControl {
    fn from(val: bool) -> Self {
        let v:usize = if val {1} else {0};
        let value = Some(v);
        WidowControl { value }
    }
}

__xml_test_suites!(
    WidowControl,
    WidowControl::default(),
    r#"<w:widowControl/>"#,
    WidowControl::from(false),
    r#"<w:widowControl w:val="0"/>"#,
    WidowControl::from(true),
    r#"<w:widowControl w:val="1"/>"#,
);
