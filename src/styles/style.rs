#![allow(unused_must_use)]
use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __string_enum, __xml_test_suites,
    formatting::{CharacterProperty, ParagraphProperty, TableProperty},
};

use crate::styles::priority::Priority;

/// Style
///
/// A style that applied to a region of the document.
///
/// ```rust
/// use docx::formatting::*;
/// use docx::styles::*;
///
/// let style = Style::new(StyleType::Paragraph, "style_id")
///     .name("Style Name")
///     .paragraph(ParagraphProperty::default())
///     .character(CharacterProperty::default());
/// ```
#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:style")]
pub struct Style<'a> {
    /// Specifies the type of style.
    #[xml(attr = "w:type")]
    pub ty: StyleType,
    #[xml(attr = "w:default")]
    pub default: Option<usize>,
    /// Specifies the unique identifier
    ///
    /// This identifier is used throughout the document to apply style in content.
    #[xml(attr = "w:styleId")]
    pub style_id: Cow<'a, str>,
    /// Specifies the primary name
    #[xml(child = "w:name")]
    pub name: Option<StyleName<'a>>,
    #[xml(child = "w:qFormat")]
    pub q_format: Option<QFormat>,
    /// Specifies the priority.
    #[xml(child = "w:uiPriority")]
    pub priority: Option<Priority>,
    #[xml(child = "w:semiHidden")]
    pub semi_hidden: Option<super::semi_hidden::SemiHidden>,
    #[xml(child = "w:unhideWhenUsed")]
    pub unhide_when_used: Option<super::unhidden_when_used::UnhideWhenUsed>,
    /// Specifies a set of paragraph properties
    #[xml(default, child = "w:pPr")]
    pub paragraph: Option<ParagraphProperty<'a>>,
    /// Specifies a set of character properties
    #[xml(default, child = "w:rPr")]
    pub character: Option<CharacterProperty<'a>>,
    #[xml(default, child = "w:tblPr")]
    pub table: Option<TableProperty<'a>>,
}

impl<'a> Style<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(ty: StyleType, style_id: T) -> Self {
        Style {
            ty,
            style_id: style_id.into(),
            default: None,
            name: None,
            q_format : None,
            priority: None,
            semi_hidden: None,
            unhide_when_used: None,
            paragraph: None,
            character: None,
            table: None,
        }
    }

    __setter!(ty: StyleType);
    __setter!(name: Option<StyleName<'a>>);
    __setter!(paragraph: Option<ParagraphProperty<'a>>);
    __setter!(character: Option<CharacterProperty<'a>>);
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:name")]
pub struct StyleName<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a, S: Into<Cow<'a, str>>> From<S> for StyleName<'a> {
    fn from(val: S) -> Self {
        StyleName { value: val.into() }
    }
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:qFormat")]
pub struct QFormat {
    #[xml(attr = "w:val")]
    pub value: Option<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum StyleType {
    Character,
    Paragraph,
    Table,
    Numbering,
}

__string_enum! {
    StyleType {
        Character = "character",
        Paragraph = "paragraph",
        Table = "table",
        Numbering = "numbering",
    }
}

__xml_test_suites!(
    Style,
    Style::new(StyleType::Numbering, "id"),
    r#"<w:style w:type="numbering" w:styleId="id"/>"#,
    Style::new(StyleType::Table, "id").name("name"),
    r#"<w:style w:type="table" w:styleId="id"><w:name w:val="name"/></w:style>"#,
    Style::new(StyleType::Paragraph, "id"),
    r#"<w:style w:type="paragraph" w:styleId="id"/>"#,
    Style::new(StyleType::Character, "id"),
    r#"<w:style w:type="character" w:styleId="id"/>"#,
);
