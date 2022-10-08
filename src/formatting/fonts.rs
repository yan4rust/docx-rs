use std::borrow::Cow;

use strong_xml::{XmlRead, XmlWrite};

use crate::{__setter, __string_enum, __xml_test_suites};

/// Size
///
/// ```rust
/// use docx_rust::formatting::*;
///
/// let sz = Size::from(42isize);
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
    #[xml(attr = "w:cs")]
    pub custom: Option<Cow<'a, str>>,
    #[xml(attr = "w:asciiTheme")]
    pub ascii_theme: Option<Cow<'a, str>>,
    #[xml(attr = "w:eastAsiaTheme")]
    pub east_asia_theme: Option<Cow<'a, str>>,
    #[xml(attr = "w:hAnsiTheme")]
    pub h_ansi_theme: Option<Cow<'a, str>>,
    #[xml(attr = "w:cstheme")]
    pub cs_theme: Option<Cow<'a, str>>,
}

impl<'a> Fonts<'a> {
    __setter!(east_asia: Option<Cow<'a, str>>);
    __setter!(ascii: Option<Cow<'a, str>>);
    __setter!(h_ansi: Option<Cow<'a, str>>);
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum FontHint {
    Default,       //	High ANSI Font
    EastAsia,      //	East Asian Font
    ComplexScript, //	Complex Script Font
}

__string_enum! {
    FontHint {
        Default= "default",
        EastAsia = "eastAsia",
        ComplexScript = "cs",
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
