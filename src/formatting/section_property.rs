#![allow(dead_code)]
use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __string_enum,
    document::HeaderFooterReference,
    formatting::{PageCols, PageGrid, PageMargin, PageSize},
};

/// Section Properties
///
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:sectPr")]
pub struct SectionProperty<'a> {
    //#[xml(attr = "w14:paraId")]
    //pub id: Option<Cow<'a, str>>,
    //#[xml(attr = "w14:textId")]
    //pub text_id: Option<Cow<'a, str>>,
    #[xml(attr = "w:rsidR")]
    pub rsid_r: Option<Cow<'a, str>>,
    #[xml(attr = "w:rsidRDefault")]
    pub rsid_r_default: Option<Cow<'a, str>>,

    #[xml(child = "w:headerReference", child = "w:footerReference")]
    /// Specifies the content of a run
    pub header_footer_references: Vec<HeaderFooterReference<'a>>,
    ///  Section-Wide Footnote Properties
    //#[xml(child = "w:footnotePr")]
    //pub footnote_property: Option<FootnoteProperty>,
    ///  Section-Wide Endnote Properties
    //#[xml(child = "w:endnotePr")]
    //pub endnote_property: Option<EndnoteProperty>,
    ///  Section Type
    #[xml(child = "w:type")]
    pub ty: Option<SectionTypeP>,
    #[xml(child = "w:pgSz")]
    pub page_size: Option<PageSize>,
    #[xml(child = "w:pgMar")]
    pub page_margin: Option<PageMargin>,
    ///  Paper Source Information
    //#[xml(child = "w:paperSrc")]
    //pub paper_source: Option<PaperSource>,
    ///  Page Borders
    //#[xml(child = "w:pgBorders")]
    //pub page_borders: Option<PgBorders>,
    ///  Line Numbering Settings
    //#[xml(child = "w:lnNumType")]
    //pub line_numbering: Option<LnNumType>,
    ///  Page Numbering Settings
    //#[xml(child = "w:pgNumType")]
    //pub page_numbering: Option<PgNumType>,
    ///  Column Definitions
    #[xml(child = "w:cols")]
    pub cols: Option<PageCols>,
    ///  Only Allow Editing of Form Fields
    #[xml(child = "w:formProt")]
    pub form_prot: Option<FormProt>,
    /////  Vertical Text Alignment on Page
    //#[xml(child = "w:vAlign")]
    //pub v_align: Option<VAlign>,
    ///  Suppress Endnotes In Document
    #[xml(child = "w:noEndnote")]
    pub no_endnote: Option<NoEndnote>,
    ///  Different First Page Headers and Footers
    #[xml(child = "w:titlePg")]
    pub title_page: Option<TitlePage>,
    ///  Text Flow Direction
    //#[xml(child = "w:textDirection")]
    //pub text_direction: Option<TextDirection>,
    ///  Right to Left Section Layout
    #[xml(child = "w:bidi")]
    pub bidi: Option<Bidi>,
    ///  Gutter on Right Side of Page
    #[xml(child = "w:rtlGutter")]
    pub rtl_gutter: Option<RtlGutter>,
    ///  Document Grid
    #[xml(child = "w:docGrid")]
    pub grid: Option<PageGrid>,
    /////  Reference to Printer Settings Data
    //#[xml(child = "w:printerSettings")]
    //pub printer_settings: Option<PrinterSettings>,
    /// Revision Information for Section Properties
    #[xml(child = "w:sectPrChange")]
    pub revision: Option<Revision<'a>>,
}

/// Previous Section Properties
///
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:sectPr")]
pub struct PreviousSectionProperty<'a> {
    //#[xml(attr = "w14:paraId")]
    //pub id: Option<Cow<'a, str>>,
    //#[xml(attr = "w14:textId")]
    //pub text_id: Option<Cow<'a, str>>,
    #[xml(attr = "w:rsidR")]
    pub rsid_r: Option<Cow<'a, str>>,
    #[xml(attr = "w:rsidRDefault")]
    pub rsid_r_default: Option<Cow<'a, str>>,

    ///  Section-Wide Footnote Properties
    //#[xml(child = "w:footnotePr")]
    //pub footnote_property: Option<FootnoteProperty>,
    ///  Section-Wide Endnote Properties
    //#[xml(child = "w:endnotePr")]
    //pub endnote_property: Option<EndnoteProperty>,
    ///  Section Type
    #[xml(child = "w:type")]
    pub ty: Option<SectionTypeP>,
    #[xml(child = "w:pgSz")]
    pub page_size: Option<PageSize>,
    #[xml(child = "w:pgMar")]
    pub page_margin: Option<PageMargin>,
    ///  Paper Source Information
    //#[xml(child = "w:paperSrc")]
    //pub paper_source: Option<PaperSource>,
    ///  Page Borders
    //#[xml(child = "w:pgBorders")]
    //pub page_borders: Option<PgBorders>,
    ///  Line Numbering Settings
    //#[xml(child = "w:lnNumType")]
    //pub line_numbering: Option<LnNumType>,
    ///  Page Numbering Settings
    //#[xml(child = "w:pgNumType")]
    //pub page_numbering: Option<PgNumType>,
    ///  Column Definitions
    #[xml(child = "w:cols")]
    pub cols: Option<PageCols>,
    ///  Only Allow Editing of Form Fields
    #[xml(child = "w:formProt")]
    pub form_prot: Option<FormProt>,
    /////  Vertical Text Alignment on Page
    //#[xml(child = "w:vAlign")]
    //pub v_align: Option<VAlign>,
    ///  Suppress Endnotes In Document
    #[xml(child = "w:noEndnote")]
    pub no_endnote: Option<NoEndnote>,
    ///  Different First Page Headers and Footers
    #[xml(child = "w:titlePg")]
    pub title_page: Option<TitlePage>,
    ///  Text Flow Direction
    //#[xml(child = "w:textDirection")]
    //pub text_direction: Option<TextDirection>,
    ///  Right to Left Section Layout
    #[xml(child = "w:bidi")]
    pub bidi: Option<Bidi>,
    ///  Gutter on Right Side of Page
    #[xml(child = "w:rtlGutter")]
    pub rtl_gutter: Option<RtlGutter>,
    ///  Document Grid
    #[xml(child = "w:docGrid")]
    pub grid: Option<PageGrid>,
    /////  Reference to Printer Settings Data
    //#[xml(child = "w:printerSettings")]
    //pub printer_settings: Option<PrinterSettings>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:sectPrChange")]
pub struct Revision<'a> {
    #[xml(attr = "w:id")]
    pub id: usize,
    #[xml(attr = "w:author")]
    pub author: Cow<'a, str>,
    #[xml(attr = "w:date")]
    pub date: Option<Cow<'a, str>>,

    #[xml(child = "w:secPr")]
    pub section_property: Option<PreviousSectionProperty<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:type")]
pub struct SectionTypeP {
    #[xml(attr = "w:val")]
    pub ty: Option<SectionType>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:formProt")]
pub struct FormProt {
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:noEndnote")]
pub struct NoEndnote {
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:bidi")]
pub struct Bidi {
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:rtlGutter")]
pub struct RtlGutter {
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum SectionType {
    NextPage,
    NextColumn,
    Continuous,
    EvenPage,
    OddPage,
}

__string_enum! {
    SectionType {
        NextPage = "nextPage",
        NextColumn = "nextColumn",
        Continuous = "continuous",
        EvenPage = "evenPage",
        OddPage = "oddPage",
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:titlePg")]
pub struct TitlePage {}

impl<'a> SectionProperty<'a> {
    //     __setter!(style_id: Option<SectionStyleId<'a>>);
    //     __setter!(justification: Option<Justification>);
    //     __setter!(border: Option<Borders<'a>>);
    //     __setter!(numbering: Option<NumberingProperty>);
    //     __setter!(spacing: Option<Spacing>);
    //     __setter!(indent: Option<Indent>);
    fn first_page_has_diffrent_header_and_footer(&mut self, val: bool) -> &mut Self {
        if val {
            self.title_page = Some(TitlePage::default());
        } else {
            self.title_page = None;
        }
        self
    }
}

// #[derive(Debug, XmlRead, XmlWrite, Clone)]
// #[cfg_attr(test, derive(PartialEq))]
// #[xml(tag = "w:pStyle")]
// pub struct SectionStyleId<'a> {
//     #[xml(attr = "w:val")]
//     pub value: Cow<'a, str>,
// }

// impl<'a, T: Into<Cow<'a, str>>> From<T> for SectionStyleId<'a> {
//     fn from(val: T) -> Self {
//         SectionStyleId { value: val.into() }
//     }
// }

// #[cfg(test)]
// use crate::formatting::JustificationVal;

// __xml_test_suites!(
//     SectionProperty,
//     SectionProperty::default(),
//     r#"<w:pPr/>"#,
//     SectionProperty::default().style_id("id"),
//     r#"<w:pPr><w:pStyle w:val="id"/></w:pPr>"#,
//     SectionProperty::default().justification(JustificationVal::Start),
//     r#"<w:pPr><w:jc w:val="start"/></w:pPr>"#,
//     SectionProperty::default().border(Borders::default()),
//     r#"<w:pPr><w:pBdr/></w:pPr>"#,
//     SectionProperty::default().numbering(NumberingProperty::default()),
//     r#"<w:pPr><w:numPr><w:numId w:val="0"/><w:ilvl w:val="0"/></w:numPr></w:pPr>"#,
// );
