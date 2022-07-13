//! Theme part
//!
//! The corresponding ZIP item is `/word/theme/theme{n}.xml`.
//!
#![allow(unused_must_use)]
#![allow(unused_imports)]
use std::borrow::Cow;
use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};

use crate::schema::{SCHEMA_DRAWINGML, SCHEMA_MAIN, SCHEMA_WORDML_14};
use crate::{__xml_test_suites, write_attr};

/// The root element of the main document part.
#[derive(Debug, Default, XmlRead, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:theme")]
pub struct Theme<'a> {
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "a:themeElements")]
    pub elements: ThemeElements<'a>,
    #[xml(child = "a:objectDefaults")]
    pub defaults: Option<ObjectDefaults>,
    #[xml(child = "a:extraClrSchemeLst")]
    pub extra_clr_scheme_lst: Option<ExtraClrSchemeLst>,
    //#[xml(child = "a:custClrLst")]
    //pub cust_clr_lst: Option<CustClrLst>,
    #[xml(child = "a:extLst")]
    pub ext_lst: Option<ExtLst>,
}

// #[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
// #[cfg_attr(test, derive(PartialEq))]
// #[xml(tag = "a:custClrLst")]
// pub struct CustClrLst<'a> {
//     #[xml(child = "a:custClr")]
//     pub contents: Vec<CustClr>,
// }

// #[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
// #[cfg_attr(test, derive(PartialEq))]
// #[xml(tag = "a:custClr")]
// pub struct CustClr<'a> {
//     #[xml(attr = "name")]
//     pub name: Option<Cow<'a, str>>,
// }

// #[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
// #[cfg_attr(test, derive(PartialEq))]
// pub enum CustClrChoice {
//     ///  RGB Color Model - Percentage Variant
//     #[xml(tag = "a:scrgbClr")]
//     ScrgbClr(ScrgbClr<'a>),
//     ///  RGB Color Model - Hex Variant
//     #[xml(tag = "a:srgbClr")]
//     SrgbClr(SrgbClr<'a>),
//     ///  Hue, Saturation, Luminance Color Model
//     #[xml(tag = "a:hslClr")]
//     HslClr(SslClr<'a>),
//     ///  System Color
//     #[xml(tag = "a:sysClr")]
//     SysClr(SysClr<'a>),
//     ///  Scheme Color
//     #[xml(tag = "a:schemeClr")]
//     SchemeClr(SchemeClr<'a>),
//     ///  Preset Color
//     #[xml(tag = "a:prstClr")]
//     PrstClr(PrstClr<'a>),
// }

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:themeElements")]
pub struct ThemeElements<'a> {
    #[xml(child = "a:clrScheme")]
    pub clr_scheme: ClrScheme<'a>,
    #[xml(child = "a:fontScheme")]
    pub font_scheme: FontScheme<'a>,
    #[xml(child = "a:fmtScheme")]
    pub fmt_scheme: FmtScheme<'a>,
    #[xml(child = "a:extLst")]
    pub ext_lst: Option<ExtLst>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:clrScheme")]
pub struct ClrScheme<'a> {
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:fontScheme")]
pub struct FontScheme<'a> {
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "a:majorFont")]
    pub major_font: MajorFont<'a>,
    #[xml(child = "a:minorFont")]
    pub minor_font: MinorFont<'a>,
    #[xml(child = "a:extLst")]
    pub ext_lst: Option<ExtLst>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:majorFont")]
pub struct MajorFont<'a> {
    #[xml(child = "a:latin")]
    pub latin: Latin<'a>,
    #[xml(child = "a:ea")]
    pub ea: EA<'a>,
    #[xml(child = "a:cs")]
    pub cs: CS<'a>,
    #[xml(child = "a:font")]
    pub fonts: Vec<Font<'a>>,
    #[xml(child = "a:extLst")]
    pub ext_lst: Option<ExtLst>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:minorFont")]
pub struct MinorFont<'a> {
    #[xml(child = "a:latin")]
    pub latin: Latin<'a>,
    #[xml(child = "a:ea")]
    pub ea: EA<'a>,
    #[xml(child = "a:cs")]
    pub cs: CS<'a>,
    #[xml(child = "a:font")]
    pub fonts: Vec<Font<'a>>,
    #[xml(child = "a:extLst")]
    pub ext_lst: Option<ExtLst>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:ea")]
pub struct EA<'a> {
    #[xml(attr = "typeface")]
    pub typeface: Option<Cow<'a, str>>,
    #[xml(attr = "panose")]
    pub panose: Option<Cow<'a, str>>,
    #[xml(attr = "pitchFamily")]
    pub pitch_family: Option<i8>,
    #[xml(attr = "charset")]
    pub charset: Option<i8>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:cs")]
pub struct CS<'a> {
    #[xml(attr = "typeface")]
    pub typeface: Option<Cow<'a, str>>,
    #[xml(attr = "panose")]
    pub panose: Option<Cow<'a, str>>,
    #[xml(attr = "pitchFamily")]
    pub pitch_family: Option<i8>,
    #[xml(attr = "charset")]
    pub charset: Option<i8>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:latin")]
pub struct Latin<'a> {
    #[xml(attr = "typeface")]
    pub typeface: Option<Cow<'a, str>>,
    #[xml(attr = "panose")]
    pub panose: Option<Cow<'a, str>>,
    #[xml(attr = "pitchFamily")]
    pub pitch_family: Option<i8>,
    #[xml(attr = "charset")]
    pub charset: Option<i8>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:font")]
pub struct Font<'a> {
    #[xml(attr = "script")]
    pub script: Cow<'a, str>,
    #[xml(attr = "typeface")]
    pub typeface: Cow<'a, str>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:fmtScheme")]
pub struct FmtScheme<'a> {
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "a:fillStyleLst")]
    pub fill_style_lst: FillStyleLst,
    #[xml(child = "a:lnStyleLst")]
    pub in_style_lst: InStyleLst,
    #[xml(child = "a:effectStyleLst")]
    pub effect_style_lst: EffectStyleLst,
    #[xml(child = "a:bgFillStyleLst")]
    pub bg_fill_style_lst: BgFillStyleLst,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:fillStyleLst")]
pub struct FillStyleLst {}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:lnStyleLst")]
pub struct InStyleLst {}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:effectStyleLst")]
pub struct EffectStyleLst {}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:bgFillStyleLst")]
pub struct BgFillStyleLst {}

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

        elements.to_writer(writer)?;
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
        r#"{}<a:theme xmlns:a="{}"><a:themeElements><a:clrScheme name=""/><a:fontScheme><a:majorFont><a:latin/><a:ea/><a:cs/></a:majorFont><a:minorFont><a:latin/><a:ea/><a:cs/></a:minorFont></a:fontScheme><a:fmtScheme><a:fillStyleLst/><a:lnStyleLst/><a:effectStyleLst/><a:bgFillStyleLst/></a:fmtScheme></a:themeElements></a:theme>"#,
        crate::schema::SCHEMA_XML,
        SCHEMA_DRAWINGML,
    )
    .as_str(),
);
