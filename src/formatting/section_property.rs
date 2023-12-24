#![allow(dead_code)]
use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __string_enum,
    document::HeaderFooterReference,
    formatting::{PageCols, PageGrid, PageMargin, PageSize},
};

use super::Bidi;

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
    #[xml(child = "w:footnotePr")]
    pub footnote_property: Option<FootnoteProperty>,
    ///  Section-Wide Endnote Properties
    #[xml(child = "w:endnotePr")]
    pub endnote_property: Option<EndnoteProperty>,
    ///  Section Type
    #[xml(child = "w:type")]
    pub ty: Option<SectionTypeP>,
    #[xml(child = "w:pgSz")]
    pub page_size: Option<PageSize>,
    #[xml(child = "w:pgMar")]
    pub page_margin: Option<PageMargin>,
    ///  Paper Source Information
    #[xml(child = "w:paperSrc")]
    pub paper_source: Option<PaperSource>,
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
    #[xml(child = "w:vAlign")]
    pub v_align: Option<VAlign>,
    ///  Suppress Endnotes In Document
    #[xml(child = "w:noEndnote")]
    pub no_endnote: Option<NoEndnote>,
    ///  Different First Page Headers and Footers
    #[xml(child = "w:titlePg")]
    pub title_page: Option<TitlePage>,
    ///  Text Flow Direction
    #[xml(child = "w:textDirection")]
    pub text_direction: Option<TextDirection>,
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
    #[xml(child = "w:footnotePr")]
    pub footnote_property: Option<FootnoteProperty>,
    ///  Section-Wide Endnote Properties
    #[xml(child = "w:endnotePr")]
    pub endnote_property: Option<EndnoteProperty>,
    ///  Section Type
    #[xml(child = "w:type")]
    pub ty: Option<SectionTypeP>,
    #[xml(child = "w:pgSz")]
    pub page_size: Option<PageSize>,
    #[xml(child = "w:pgMar")]
    pub page_margin: Option<PageMargin>,
    ///  Paper Source Information
    #[xml(child = "w:paperSrc")]
    pub paper_source: Option<PaperSource>,
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
    #[xml(child = "w:vAlign")]
    pub v_align: Option<VAlign>,
    ///  Suppress Endnotes In Document
    #[xml(child = "w:noEndnote")]
    pub no_endnote: Option<NoEndnote>,
    ///  Different First Page Headers and Footers
    #[xml(child = "w:titlePg")]
    pub title_page: Option<TitlePage>,
    ///  Text Flow Direction
    #[xml(child = "w:textDirection")]
    pub text_direction: Option<TextDirection>,
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
    pub id: isize,
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

// #[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
// #[cfg_attr(test, derive(PartialEq))]
// #[xml(tag = "w:bidi")]
// pub struct Bidi {
//     #[xml(attr = "w:val")]
//     pub val: Option<bool>,
// }

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:paperSrc")]
pub struct PaperSource {
    #[xml(attr = "w:first")]
    pub first: Option<isize>,
    #[xml(attr = "w:other")]
    pub other: Option<isize>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:textDirection")]
pub struct TextDirection {
    #[xml(attr = "w:val")]
    pub val: TextDirectionType,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TextDirectionType {
    #[default]
    LrTb, //	Left to Right, Top to Bottom
    TbRl,  //		Top to Bottom, Right to Left
    BtLr,  //		Bottom to Top, Left to Right
    LrTbV, //		Left to Right, Top to Bottom Rotated
    TbRlV, //		Top to Bottom, Right to Left Rotated
    TbLrV, //		Top to Bottom, Left to Right Rotated
}

__string_enum! {
    TextDirectionType {
        LrTb = "lrTb",
        TbRl = "tbRl",
        BtLr = "btLr",
        LrTbV = "lrTbV",
        TbRlV = "tbRlV",
        TbLrV = "tbLrV",
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:vAlign")]
pub struct VAlign {
    #[xml(attr = "w:val")]
    pub val: TextDirectionType,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum VAlignType {
    #[default]
    Top, //	Align Top
    Center, //	Align Center
    Both,   //	Vertical Justification
    Bottom, //	Align Bottom
}

__string_enum! {
    VAlignType {
        Top = "top",
        Center = "center",
        Both = "both",
        Bottom = "bottom",
    }
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

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:footnotePr")]
pub struct FootnoteProperty {
    #[xml(child = "w:pos")]
    pub position: Option<FootnotePosition>,
    #[xml(child = "w:numFmt")]
    pub num_fmt: Option<NumFmt>,
    /// Footnote and Endnote Numbering Starting Value
    #[xml(child = "w:numStart")]
    pub num_start: Option<NumStart>,
    /// Footnote and Endnote Numbering Restart Location
    #[xml(child = "w:numRestart")]
    pub num_restart: Option<NumRestart>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:endnotePr")]
pub struct EndnoteProperty {
    #[xml(child = "w:pos")]
    pub position: Option<EndnotePosition>,
    #[xml(child = "w:numFmt")]
    pub num_fmt: Option<NumFmt>,
    /// Footnote and Endnote Numbering Starting Value
    #[xml(child = "w:numStart")]
    pub num_start: Option<NumStart>,
    /// Footnote and Endnote Numbering Restart Location
    #[xml(child = "w:numRestart")]
    pub num_restart: Option<NumRestart>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:footnotePr")]
pub struct FootnoteProperty2 {
    #[xml(child = "w:pos")]
    pub position: Option<FootnotePosition>,
    #[xml(child = "w:numFmt")]
    pub num_fmt: Option<NumFmt>,
    /// Footnote and Endnote Numbering Starting Value
    #[xml(child = "w:numStart")]
    pub num_start: Option<NumStart>,
    /// Footnote and Endnote Numbering Restart Location
    #[xml(child = "w:numRestart")]
    pub num_restart: Option<NumRestart>,
    #[xml(child = "w:footnote")]
    pub footnote: Vec<Footnote>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:footnote")]
pub struct Footnote {
    #[xml(attr = "w:id")]
    pub id: isize,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:endnotePr")]
pub struct EndnoteProperty2 {
    #[xml(child = "w:pos")]
    pub position: Option<EndnotePosition>,
    #[xml(child = "w:numFmt")]
    pub num_fmt: Option<NumFmt>,
    /// Footnote and Endnote Numbering Starting Value
    #[xml(child = "w:numStart")]
    pub num_start: Option<NumStart>,
    /// Footnote and Endnote Numbering Restart Location
    #[xml(child = "w:numRestart")]
    pub num_restart: Option<NumRestart>,
    #[xml(child = "w:endnote")]
    pub endnote: Vec<Endnote>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:endnote")]
pub struct Endnote {
    #[xml(attr = "w:id")]
    pub id: isize,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:numRestart")]
pub struct NumRestart {
    #[xml(attr = "w:val")]
    pub val: NumRestartType,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum NumRestartType {
    #[default]
    Continuous, //	Continue Numbering From Previous Section
    EachSect, //	Restart Numbering For Each Section
    EachPage, //	Restart Numbering On Each Page
}

__string_enum! {
    NumRestartType {
        Continuous = "continuous",
        EachSect = "eachSect",
        EachPage = "eachPage",
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:numStart")]
pub struct NumStart {
    #[xml(attr = "w:val")]
    pub val: Option<isize>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:numFmt")]
pub struct NumFmt {
    #[xml(attr = "w:val")]
    pub ty: NumFmtType,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum NumFmtType {
    #[default]
    Decimal, //Decimal Numbers.
    UpperRoman,                   //Uppercase Roman Numerals.
    LowerRoman,                   //Lowercase Roman Numerals.
    UpperLetter,                  //Uppercase Latin Alphabet.
    LowerLetter,                  //Lowercase Latin Alphabet.
    Ordinal,                      //Ordinal.
    CardinalText,                 //Cardinal Text.
    OrdinalText,                  //Ordinal Text.
    Hex,                          //Hexadecimal Numbering.
    Chicago,                      //Chicago Manual of Style.
    IdeographDigital,             //Ideographs.
    JapaneseCounting,             //Japanese Counting System.
    Aiueo,                        //AIUEO Order Hiragana.
    Iroha,                        //Iroha Ordered Katakana.
    DecimalFullWidth,             //Double Byte Arabic Numerals.
    DecimalHalfWidth,             //Single Byte Arabic Numerals.
    JapaneseLegal,                //Japanese Legal Numbering.
    JapaneseDigitalTenThousand,   //Japanese Digital Ten Thousand Counting System.
    DecimalEnclosedCircle,        //Decimal Numbers Enclosed in a Circle.
    DecimalFullWidth2,            //Double Byte Arabic Numerals Alternate.
    AiueoFullWidth,               //Full-Width AIUEO Order Hiragana.
    IrohaFullWidth,               //Full-Width Iroha Ordered Katakana.
    DecimalZero,                  //Initial Zero Arabic Numerals.
    Bullet,                       //Bullet.
    Ganada,                       //Korean Ganada Numbering.
    Chosung,                      //Korean Chosung Numbering.
    DecimalEnclosedFullstop,      //Decimal Numbers Followed by a Period.
    DecimalEnclosedParen,         //Decimal Numbers Enclosed in Parenthesis.
    DecimalEnclosedCircleChinese, //Decimal Numbers Enclosed in a Circle.
    IdeographEnclosedCircle,      //Ideographs Enclosed in a Circle.
    IdeographTraditional,         //Traditional Ideograph Format.
    IdeographZodiac,              //Zodiac Ideograph Format.
    IdeographZodiacTraditional,   //Traditional Zodiac Ideograph Format.
    TaiwaneseCounting,            //Taiwanese Counting System.
    IdeographLegalTraditional,    //Traditional Legal Ideograph Format.
    TaiwaneseCountingThousand,    //Taiwanese Counting Thousand System.
    TaiwaneseDigital,             //Taiwanese Digital Counting System.
    ChineseCounting,              //Chinese Counting System.
    ChineseLegalSimplified,       //Chinese Legal Simplified Format.
    ChineseCountingThousand,      //Chinese Counting Thousand System.
    KoreanDigital,                //Korean Digital Counting System.
    KoreanCounting,               //Korean Counting System.
    KoreanLegal,                  //Korean Legal Numbering.
    KoreanDigital2,               //Korean Digital Counting System Alternate.
    VietnameseCounting,           //Vietnamese Numerals.
    RussianLower,                 //Lowercase Russian Alphabet.
    RussianUpper,                 //Uppercase Russian Alphabet.
    None,                         //No Numbering.
    NumberInDash,                 //Number With Dashes.
    Hebrew1,                      //Hebrew Numerals.
    Hebrew2,                      //Hebrew Alphabet.
    ArabicAlpha,                  //Arabic Alphabet.
    ArabicAbjad,                  //Arabic Abjad Numerals.
    HindiVowels,                  //Hindi Vowels.
    HindiConsonants,              //Hindi Consonants.
    HindiNumbers,                 //Hindi Numbers.
    HindiCounting,                //Hindi Counting System.
    ThaiLetters,                  //Thai Letters.
    ThaiNumbers,                  //Thai Numerals.
    ThaiCounting,                 //Thai Counting System.
}

__string_enum! {
    NumFmtType {
        Decimal = "decimal",
        UpperRoman = "upperRoman",
        LowerRoman = "lowerRoman",
        UpperLetter = "upperLetter",
        LowerLetter = "lowerLetter",
        Ordinal = "ordinal",
        CardinalText = "cardinalText",
        OrdinalText = "ordinalText",
        Hex = "hex",
        Chicago = "chicago",
        IdeographDigital = "ideographDigital",
        JapaneseCounting = "japaneseCounting",
        Aiueo = "aiueo",
        Iroha = "iroha",
        DecimalFullWidth = "decimalFullWidth",
        DecimalHalfWidth = "decimalHalfWidth",
        JapaneseLegal = "japaneseLegal",
        JapaneseDigitalTenThousand = "japaneseDigitalTenThousand",
        DecimalEnclosedCircle = "decimalEnclosedCircle",
        DecimalFullWidth2 = "decimalFullWidth2",
        AiueoFullWidth = "aiueoFullWidth",
        IrohaFullWidth = "irohaFullWidth",
        DecimalZero = "decimalZero",
        Bullet = "bullet",
        Ganada = "ganada",
        Chosung = "chosung",
        DecimalEnclosedFullstop = "decimalEnclosedFullstop",
        DecimalEnclosedParen = "decimalEnclosedParen",
        DecimalEnclosedCircleChinese = "decimalEnclosedCircleChinese",
        IdeographEnclosedCircle = "ideographEnclosedCircle",
        IdeographTraditional = "ideographTraditional",
        IdeographZodiac = "ideographZodiac",
        IdeographZodiacTraditional = "ideographZodiacTraditional",
        TaiwaneseCounting = "taiwaneseCounting",
        IdeographLegalTraditional = "ideographLegalTraditional",
        TaiwaneseCountingThousand = "taiwaneseCountingThousand",
        TaiwaneseDigital = "taiwaneseDigital",
        ChineseCounting = "chineseCounting",
        ChineseLegalSimplified = "chineseLegalSimplified",
        ChineseCountingThousand = "chineseCountingThousand",
        KoreanDigital = "koreanDigital",
        KoreanCounting = "koreanCounting",
        KoreanLegal = "koreanLegal",
        KoreanDigital2 = "koreanDigital2",
        VietnameseCounting = "vietnameseCounting",
        RussianLower = "russianLower",
        RussianUpper = "russianUpper",
        None = "none",
        NumberInDash = "numberInDash",
        Hebrew1 = "hebrew1",
        Hebrew2 = "hebrew2",
        ArabicAlpha = "arabicAlpha",
        ArabicAbjad = "arabicAbjad",
        HindiVowels = "hindiVowels",
        HindiConsonants = "hindiConsonants",
        HindiNumbers = "hindiNumbers",
        HindiCounting = "hindiCounting",
        ThaiLetters = "thaiLetters",
        ThaiNumbers = "thaiNumbers",
        ThaiCounting = "thaiCounting",
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:pos")]
pub struct FootnotePosition {
    #[xml(attr = "w:val")]
    pub val: PositionType,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum PositionType {
    #[default]
    PageBottom, //	Footnotes Positioned at Page Bottom
    BeneathText, //	Footnotes Positioned Beneath Text
    SectEnd,     //	Footnotes Positioned At End of Section
    DocEnd,      //	Footnotes Positioned At End of Document
}

__string_enum! {
    PositionType {
        PageBottom = "pageBottom",
        BeneathText = "beneathText",
        SectEnd = "sectEnd",
        DocEnd = "docEnd",
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:pos")]
pub struct EndnotePosition {
    #[xml(attr = "w:val")]
    pub val: EndnotePositionType,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum EndnotePositionType {
    #[default]
    SectEnd, //	Footnotes Positioned At End of Section
    DocEnd, //	Footnotes Positioned At End of Document
}

__string_enum! {
    EndnotePositionType {
        SectEnd = "sectEnd",
        DocEnd = "docEnd",
    }
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
