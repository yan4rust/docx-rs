use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __xml_test_suites,
    formatting::{Borders, Indent, PageCols, PageGrid, PageSize, PageMargin, Justification, NumberingProperty, Spacing, WidowControl}, document::HeaderFooterReference,
};

/// Section Property
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
    #[xml(
        child = "w:headerReference",
        child = "w:footerReference",
    )]
    /// Specifies the content of a run
    pub header_footer_references: Vec<HeaderFooterReference<'a>>,
    #[xml(child = "w:pgSz")]
    pub page_size: Option<PageSize>,
    #[xml(child = "w:pgMar")]
    pub page_margin: Option<PageMargin>,
    #[xml(child = "w:cols")]
    pub cols: Option<PageCols>,
    #[xml(child = "w:titlePg")]
    pub title_page: Option<TitlePage>,
    #[xml(child = "w:docGrid")]
    pub grid: Option<PageGrid<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:titlePg")]
pub struct TitlePage {
    
}

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
