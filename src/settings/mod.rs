//! Settings part
//!
//! The corresponding ZIP item is `/word/settings.xml`.
//!

use std::borrow::Cow;
use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};

use crate::{__xml_test_suites, write_attr};
use crate::schema::{SCHEMA_MAIN, SCHEMA_WORDML_14};

/// The root element of the main document part.
#[derive(Debug, Default, XmlRead, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:settings")]
pub struct Settings<'a> {
    #[xml(child = "w:evenAndOddHeaders")]
    pub even_and_odd_headers: Option<EvenAndOddHeaders>,
    #[xml(child = "w:themeFontLang")]
    pub theme_font_lang: Option<ThemeFontLang<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:evenAndOddHeaders")]
pub struct EvenAndOddHeaders {}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:themeFontLang")]
pub struct ThemeFontLang<'a> {
    #[xml(attr = "w:val")]
    pub val: Option<Cow<'a, str>>,
    #[xml(attr = "w:eastAsia")]
    pub east_asia: Option<Cow<'a, str>>,
}

impl<'a> XmlWrite for Settings<'a> {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let Settings {
            even_and_odd_headers,
            theme_font_lang,
        } = self;

        log::debug!("[Settings] Started writing.");
        let _ = write!(writer.inner, "{}", crate::schema::SCHEMA_XML);

        writer.write_element_start("w:settings")?;

        writer.write_attribute("xmlns:w", SCHEMA_MAIN)?;

        writer.write_attribute("xmlns:w14", SCHEMA_WORDML_14)?;

        writer.write_element_end_open()?;

        write_attr(even_and_odd_headers, writer)?;

        write_attr(theme_font_lang, writer)?;

        writer.write_element_end_close("w:settings")?;

        log::debug!("[Settings] Finished writing.");

        Ok(())
    }
}

__xml_test_suites!(
    Settings,
    Settings::default(),
    format!(
        r#"{}<w:settings xmlns:w="{}" xmlns:w14="{}"></w:settings>"#,
        crate::schema::SCHEMA_XML,
        SCHEMA_MAIN,
        SCHEMA_WORDML_14
    )
    .as_str(),
);
