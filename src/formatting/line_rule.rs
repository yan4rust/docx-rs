use crate::__string_enum;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum LineRule {
    Auto,
}

__string_enum! {
    LineRule {
        Auto = "auto",
    }
}
