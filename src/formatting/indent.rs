#![allow(dead_code)]

use strong_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

/// indent
/// percent to one character.
///
/// ```rust
/// use docx::formatting::*;
///
/// let sz = Size::from(42usize);
/// ```
#[derive(Debug, XmlRead, XmlWrite, Clone, Default)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:ind")]
pub struct Indent {
    #[xml(attr = "w:leftChars")]
    pub left_chars: Option<usize>,
    #[xml(attr = "w:left")]
    pub left: Option<usize>,
    #[xml(attr = "w:rightChars")]
    pub right_chars: Option<usize>,
    #[xml(attr = "w:right")]
    pub right: Option<usize>,
    #[xml(attr = "w:firstLineChars")]
    pub first_line_chars: Option<usize>,
    #[xml(attr = "w:firstLine")]
    pub first_line: Option<usize>,
}

impl Indent {
    fn left<T>(mut self, val: T) -> Self
    where
        usize: From<T>,
    {
        self.left_chars = Some(val.into());
        self.left = self.left_chars;
        self
    }

    fn right<T>(mut self, val: T) -> Self
    where
        usize: From<T>,
    {
        self.right_chars = Some(val.into());
        self.right = self.right_chars;
        self
    }

    fn first_line<T>(mut self, val: T) -> Self
    where
        usize: From<T>,
    {
        self.first_line = Some(val.into());
        self.first_line_chars = self.first_line;
        self
    }
}

__xml_test_suites!(
    Indent,
    Indent::default().first_line(200usize),
    r#"<w:ind w:firstLineChars="200" w:firstLine="200"/>"#,
);
