#![allow(unused_must_use)]
use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

/// Style
///
/// A style that applied to a region of the document.
///
/// ```rust
/// use docx_rust::formatting::*;
/// use docx_rust::styles::*;
///
/// let style = Style::new(StyleType::Paragraph, "style_id")
///     .name("Style Name")
///     .paragraph(ParagraphProperty::default())
///     .character(CharacterProperty::default());
/// ```
#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:lsdException")]
pub struct LatentStyle<'a> {
    /// Specifies the type of style.
    #[xml(attr = "w:name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "w:semiHidden")]
    pub semi_hidden: Option<usize>,
    #[xml(attr = "w:uiPriority")]
    pub priority: Option<usize>,
    #[xml(attr = "w:unhideWhenUsed")]
    pub unhiden_when_used: Option<usize>,
    #[xml(attr = "w:qFormat")]
    pub q_format: Option<usize>,
}
