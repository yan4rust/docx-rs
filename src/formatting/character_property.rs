use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __xml_test_suites,
    formatting::{Bold, Color, Dstrike, Fonts, Italics, Lang, Outline, Size, Strike, Underline},
};

use super::{BoldComplex, Caps, Highlight, ItalicsComplex, Position, SmallCaps, VertAlign};

/// Character Property
///
/// ```rust
/// use docx_rust::formatting::{CharacterProperty, UnderlineStyle};
///
/// let prop = CharacterProperty::default()
///     .style_id("foo")
///     .color("00ff00")
///     .color(0xff0000)
///     .color((0x00, 0x00, 0xff))
///     .size(42usize)
///     .bold(true)
///     .italics(false)
///     .strike(true)
///     .dstrike(false)
///     .outline(true)
///     .underline("00ff00")
///     .underline(("ff0000", UnderlineStyle::Dash));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:rPr")]
pub struct CharacterProperty<'a> {
    /// Specifies the style ID of the character style.
    #[xml(child = "w:rStyle")]
    pub style_id: Option<CharacterStyleId<'a>>,
    /// Specifies the font.
    #[xml(child = "w:rFonts")]
    pub fonts: Option<Fonts<'a>>,
    /// Specifies that the text of the text run is to be bold.
    #[xml(child = "w:b")]
    pub bold: Option<Bold>,
    #[xml(child = "w:bCs")]
    pub bold_complex: Option<BoldComplex>,
    /// Specifies that the text of the text run is to be italics.
    #[xml(child = "w:i")]
    pub italics: Option<Italics>,
    #[xml(child = "w:iCs")]
    pub italics_complex: Option<ItalicsComplex>,
    /// Display All Characters As Capital Letters
    #[xml(child = "w:caps")]
    pub caps: Option<Caps>,
    #[xml(child = "w:smallCaps")]
    pub small_caps: Option<SmallCaps>,
    /// Specifies that the contents are to be displayed with a horizontal line through the center of the line.
    #[xml(child = "w:strike")]
    pub strike: Option<Strike>,
    /// Specifies that the contents are to be displayed with two horizontal lines through each character.
    #[xml(child = "w:dstrike")]
    pub dstrike: Option<Dstrike>,
    /// Specifies that the content should be displayed as if it had an outline.
    #[xml(child = "w:outline")]
    pub outline: Option<Outline>,
    /// Specifies the color to be used to display text.
    #[xml(child = "w:color")]
    pub color: Option<Color<'a>>,
    /// Vertically Raised or Lowered Text
    #[xml(child = "w:position")]
    pub position: Option<Position>,
    /// Specifies the font size in half points.
    #[xml(child = "w:sz")]
    pub size: Option<Size>,
    #[xml(child = "w:highlight")]
    pub highlight: Option<Highlight>,
    /// Specifies that the content should be displayed with an underline
    #[xml(child = "w:u")]
    pub underline: Option<Underline<'a>>,
    #[xml(child = "w:vertAlign")]
    pub vertical_align: Option<VertAlign>,
    /// Specifies the language to be used.
    #[xml(child = "w:lang")]
    pub lang: Option<Lang<'a>>,
}

impl<'a> CharacterProperty<'a> {
    __setter!(style_id: Option<CharacterStyleId<'a>>);
    __setter!(color: Option<Color<'a>>);
    __setter!(bold: Option<Bold>);
    __setter!(dstrike: Option<Dstrike>);
    __setter!(italics: Option<Italics>);
    __setter!(outline: Option<Outline>);
    __setter!(strike: Option<Strike>);
    __setter!(size: Option<Size>);
    __setter!(underline: Option<Underline<'a>>);
    __setter!(fonts: Option<Fonts<'a>>);
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:rStyle")]
pub struct CharacterStyleId<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for CharacterStyleId<'a> {
    fn from(val: T) -> Self {
        CharacterStyleId { value: val.into() }
    }
}

__xml_test_suites!(
    CharacterProperty,
    CharacterProperty::default(),
    r#"<w:rPr/>"#,
    CharacterProperty::default().style_id("id"),
    r#"<w:rPr><w:rStyle w:val="id"/></w:rPr>"#,
    CharacterProperty::default().color("00ff00"),
    r#"<w:rPr><w:color w:val="00ff00"/></w:rPr>"#,
    CharacterProperty::default().size(42usize),
    r#"<w:rPr><w:sz w:val="42"/></w:rPr>"#,
    CharacterProperty::default().bold(true),
    r#"<w:rPr><w:b w:val="true"/></w:rPr>"#,
    CharacterProperty::default().italics(false),
    r#"<w:rPr><w:i w:val="false"/></w:rPr>"#,
    CharacterProperty::default().outline(true),
    r#"<w:rPr><w:outline w:val="true"/></w:rPr>"#,
    CharacterProperty::default().strike(false),
    r#"<w:rPr><w:strike w:val="false"/></w:rPr>"#,
    CharacterProperty::default().dstrike(true),
    r#"<w:rPr><w:dstrike w:val="true"/></w:rPr>"#,
    CharacterProperty::default().underline(Underline::default()),
    r#"<w:rPr><w:u/></w:rPr>"#,
    CharacterProperty::default().fonts(Fonts::default().east_asia("宋体")),
    r#"<w:rPr><w:rFonts w:eastAsia="宋体"/></w:rPr>"#,
);
