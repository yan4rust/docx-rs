use strong_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tcPr")]
pub struct TableCellProperty {
    #[xml(child = "w:tcW")]
    pub wide: Option<super::TableCellWidth>,
}

impl TableCellProperty {}

__xml_test_suites!(
    TableCellProperty,
    TableCellProperty::default(),
    r#"<w:tcPr/>"#,
);
