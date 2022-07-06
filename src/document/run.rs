#![allow(unused_must_use)]
use derive_more::From;
use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __xml_test_suites,
    document::{field_char::FieldChar, instrtext::InstrText, r#break::Break, text::Text},
    formatting::CharacterProperty,
    DocxResult,
};

/// Run
///
/// Run is a non-block region of text with properties.
///
/// ```rust
/// use docx::document::*;
/// use docx::formatting::*;
///
/// let run = Run::default()
///     .property(CharacterProperty::default())
///     .push_text("text")
///     .push_break(None)
///     .push_text((" text ", TextSpace::Preserve))
///     .push_break(BreakType::Column);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:r")]
pub struct Run<'a> {
    /// Specifies the properties of a run
    ///
    /// Just as paragraph, a run's properties is applied to all the contents of the run.
    #[xml(default, child = "w:rPr")]
    pub property: CharacterProperty<'a>,
    #[xml(
        child = "w:t",
        child = "w:br",
        child = "w:fldChar",
        child = "w:instrText"
    )]
    /// Specifies the content of a run
    pub content: Vec<RunContent<'a>>,
    #[xml(attr = "w:is_delete")]
    pub shall_destroy: Option<bool>,
}

impl<'a> Run<'a> {
    __setter!(property: CharacterProperty<'a>);
    __setter!(shall_destroy: Option<bool>);

    #[inline(always)]
    pub fn push<T: Into<RunContent<'a>>>(mut self, content: T) -> Self {
        self.content.push(content.into());
        self
    }

    #[inline(always)]
    pub fn push_text<T: Into<Text<'a>>>(mut self, content: T) -> Self {
        self.content.push(RunContent::Text(content.into()));
        self
    }

    #[inline(always)]
    pub fn push_break<T: Into<Break>>(mut self, br: T) -> Self {
        self.content.push(RunContent::Break(br.into()));
        self
    }

    pub fn iter_text(&self) -> impl Iterator<Item = &Cow<'a, str>> {
        self.content.iter().filter_map(|content| match content {
            RunContent::Text(Text { text, .. }) => Some(text),
            RunContent::InstrText(InstrText { text, .. }) => Some(text),
            RunContent::Break(_) => None,
            RunContent::FieldChar(_) => None,
        })
    }

    pub fn iter_text_mut(&mut self) -> impl Iterator<Item = &mut Cow<'a, str>> {
        self.content.iter_mut().filter_map(|content| match content {
            RunContent::Text(Text { text, .. }) => Some(text),
            RunContent::InstrText(InstrText { text, .. }) => Some(text),
            RunContent::Break(_) => None,
            RunContent::FieldChar(_) => None,
        })
    }

    pub fn replace_text_simple<S>(&mut self, old: S, new: S)
    where
        S: AsRef<str>,
    {
        let dic = (old, new);
        let dic = vec![dic];
        self.replace_text(&dic);
    }

    pub fn replace_text<'b, T, S>(&mut self, dic: T) -> DocxResult<()>
    where
        S: AsRef<str> + 'b,
        T: IntoIterator<Item = &'b (S, S)> + std::marker::Copy,
    {
        for c in self.content.iter_mut() {
            match c {
                RunContent::Text(t) => {
                    let mut tc = t.text.to_string();
                    for p in dic {
                        tc = tc.replace(p.0.as_ref(), p.1.as_ref());
                    }
                    t.text = tc.into();
                }
                _ => {}
            }
        }

        Ok(())
    }
}

/// A set of elements that can be contained as the content of a run.
#[derive(Debug, From, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum RunContent<'a> {
    #[xml(tag = "w:t")]
    Text(Text<'a>),
    #[xml(tag = "w:br")]
    Break(Break),
    #[xml(tag = "w:fldChar")]
    FieldChar(FieldChar),
    #[xml(tag = "w:instrText")]
    InstrText(InstrText<'a>),
}

__xml_test_suites!(
    Run,
    Run::default(),
    r#"<w:r><w:rPr/></w:r>"#,
    Run::default().push_break(None),
    r#"<w:r><w:rPr/><w:br/></w:r>"#,
    Run::default().push_text("text"),
    r#"<w:r><w:rPr/><w:t>text</w:t></w:r>"#,
);
