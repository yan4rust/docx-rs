//! Main Document part
//!
//! The corresponding ZIP item is `/word/document.xml`.
//! 

use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};

use crate::__xml_test_suites;
use crate::schema::{SCHEMA_MAIN, SCHEMA_WORDML_14};

use crate::{
    document::{Body, BodyContent},
};

/// The root element of the main document part.
#[derive(Debug, Default, XmlRead, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:document")]
pub struct Document<'a> {
    /// Specifies the body of the docment.
    #[xml(child = "w:body")]
    pub body: Body<'a>,
}

impl<'a> Document<'a> {
    pub fn push<T: Into<BodyContent<'a>>>(&mut self, content: T) -> &mut Self {
        self.body.push(content);
        self
    }
}

impl<'a> XmlWrite for Document<'a> {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let Document { body } = self;

        log::debug!("[Document] Started writing.");
        let _ = write!(writer.inner, "{}", crate::schema::SCHEMA_XML);

        writer.write_element_start("w:document")?;

        writer.write_attribute("xmlns:w", SCHEMA_MAIN)?;

        writer.write_attribute("xmlns:w14", SCHEMA_WORDML_14)?;

        writer.write_element_end_open()?;

        body.to_writer(writer)?;

        writer.write_element_end_close("w:document")?;

        log::debug!("[Document] Finished writing.");

        Ok(())
    }
}

__xml_test_suites!(
    Document,
    Document::default(),
    format!(
        r#"{}<w:document xmlns:w="{}" xmlns:w14="{}"><w:body/></w:document>"#,
        crate::schema::SCHEMA_XML,
        SCHEMA_MAIN, SCHEMA_WORDML_14
    )
    .as_str(),
);