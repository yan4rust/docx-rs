use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{__setter, __xml_test_suites, formatting::BorderStyle};

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:right")]
pub struct RightBorder<'a> {
    #[xml(attr = "w:val")]
    pub style: super::BorderStyle,
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<crate::formatting::ThemeColor>,
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<Cow<'a, str>>,
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<Cow<'a, str>>,
    #[xml(attr = "w:sz")]
    pub size: Option<usize>, // Measurement in Eighths of a Point
    #[xml(attr = "w:space")]
    pub space: Option<usize>,
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}

impl<'a> RightBorder<'a> {
    __setter!(color: Option<Cow<'a, str>>);
    __setter!(shadow: Option<bool>);
    __setter!(space: Option<usize>);
    __setter!(size: Option<usize>);
    __setter!(style: BorderStyle);
}

__xml_test_suites!(
    RightBorder,
    RightBorder::default(),
    r#"<w:right w:val="none"/>"#,
    RightBorder::default().color("000000"),
    r#"<w:right w:val="none" w:color="000000"/>"#,
    RightBorder::default().shadow(false),
    r#"<w:right w:val="none" w:shadow="false"/>"#,
    RightBorder::default().space(40usize),
    r#"<w:right w:val="none" w:space="40"/>"#,
    RightBorder::default().size(20usize),
    r#"<w:right w:val="none" w:sz="20"/>"#,
    RightBorder::default().style(BorderStyle::Dotted),
    r#"<w:right w:val="dotted"/>"#,
);
