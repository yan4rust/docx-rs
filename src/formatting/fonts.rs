use std::borrow::Cow;

use strong_xml::{XmlRead, XmlWrite};

use crate::{__setter, __string_enum, __xml_test_suites};

/// Size
///
/// ```rust
/// use docx::formatting::*;
///
/// let sz = Size::from(42usize);
/// ```
#[derive(Debug, XmlRead, XmlWrite, Clone, Default)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:rFonts")]
pub struct Fonts<'a> {
    #[xml(attr = "w:hint")]
    pub hint: Option<FontHint>,
    #[xml(attr = "w:ascii")]
    pub ascii: Option<Cow<'a, str>>,
    #[xml(attr = "w:eastAsia")]
    pub east_asia: Option<Cow<'a, str>>,
    #[xml(attr = "w:hAnsi")]
    pub h_ansi: Option<Cow<'a, str>>,
}

impl<'a> Fonts<'a> {
    __setter!(east_asia: Option<Cow<'a, str>>);
    __setter!(ascii: Option<Cow<'a, str>>);
    __setter!(h_ansi: Option<Cow<'a, str>>);
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum FontHint {
    /// Text restarts on the next column.
    Ascii,
    /// Text restarts on the next page.
    EastAsia,
    /// Text restarts on the next line.
    HAnsi,
}

__string_enum! {
    FontHint {
        Ascii= "ascii",
        EastAsia = "eastAsia",
        HAnsi = "hAnsi",
    }
}

__xml_test_suites!(
    Fonts,
    Fonts::default().east_asia("宋体"),
    r#"<w:rFonts w:eastAsia="宋体"/>"#,
    Fonts::default()
        .east_asia("宋体")
        .ascii("Batang")
        .h_ansi("Batang"),
    r#"<w:rFonts w:ascii="Batang" w:eastAsia="宋体" w:hAnsi="Batang"/>"#,
);
