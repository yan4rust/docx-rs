//! Theme part
//!
//! The corresponding ZIP item is `/word/theme/theme{n}.xml`.
//!
#![allow(unused_imports)]
use std::borrow::Cow;
use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};

use crate::{__xml_test_suites, write_attr};
use crate::schema::{SCHEMA_DRAWINGML, SCHEMA_MAIN, SCHEMA_WORDML_14};

/// The root element of the main document part.
#[derive(Debug, Default, XmlRead, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:theme")]
pub struct Theme<'a> {
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "a:themeElements")]
    pub elements: Option<ThemeElements<'a>>,
    #[xml(child = "a:objectDefaults")]
    pub defaults: Option<ObjectDefaults>,
    #[xml(child = "a:extraClrSchemeLst")]
    pub extra_clr_scheme_lst: Option<ExtraClrSchemeLst>,
    #[xml(child = "a:extLst")]
    pub ext_lst: Option<ExtLst>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:themeElements")]
pub struct ThemeElements<'a> {
    #[xml(child = "a:clrScheme")]
    pub clr_scheme: Option<ClrScheme<'a>>,
    #[xml(child = "a:fontScheme")]
    pub font_scheme: Option<FontScheme<'a>>,
    #[xml(child = "a:fmtScheme")]
    pub fmt_scheme: Option<FmtScheme<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:clrScheme")]
pub struct ClrScheme<'a> {
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:fontScheme")]
pub struct FontScheme<'a> {
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:fmtScheme")]
pub struct FmtScheme<'a> {
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:objectDefaults")]
pub struct ObjectDefaults {}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:extraClrSchemeLst")]
pub struct ExtraClrSchemeLst {}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:extLst")]
pub struct ExtLst {}

impl<'a> XmlWrite for Theme<'a> {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let Theme {
            name,
            elements,
            defaults,
            extra_clr_scheme_lst,
            ext_lst,
        } = self;

        log::debug!("[Theme] Started writing.");
        let _ = write!(writer.inner, "{}", crate::schema::SCHEMA_XML);

        writer.write_element_start("a:theme")?;

        writer.write_attribute("xmlns:a", SCHEMA_DRAWINGML)?;

        if let Some(n) = name {
            writer.write_attribute("name", n)?;
        }

        writer.write_element_end_open()?;

        write_attr(elements, writer)?;

        write_attr(defaults, writer)?;

        write_attr(extra_clr_scheme_lst, writer)?;

        write_attr(ext_lst, writer)?;

        writer.write_element_end_close("a:theme")?;

        log::debug!("[Theme] Finished writing.");

        Ok(())
    }
}

__xml_test_suites!(
    Theme,
    Theme::default(),
    format!(
        r#"{}<a:theme xmlns:a="{}"></a:theme>"#,
        crate::schema::SCHEMA_XML,
        SCHEMA_DRAWINGML,
    )
    .as_str(),
);
