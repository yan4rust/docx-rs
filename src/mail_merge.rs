use std::{collections::HashMap, path::Path};

use crate::{Docx, DocxResult, document::{Paragraph, Run}};

pub enum Part<'a> {
    Paragraph(&'a mut Paragraph<'a>),
    Run(&'a mut Run<'a>),
}

pub struct MergeGroups<'a> {
    pub name: String,
    pub contents: Vec<Part<'a>>
}

pub fn mail_merge<P>(template: &Docx, map: HashMap<String, String>, path: P) -> DocxResult<()>
where
    P: AsRef<Path>,
{
    let mut docx = template.clone();
    let dmap: HashMap<String, Vec<Part>> = HashMap::new();
    let mut is_merge_field = false;
    for c in docx.document.body.content.iter() {
        match c {
            crate::document::BodyContent::Paragraph(p) => {
                if ! is_merge_field {
                    let mut iter = p.content.iter().skip_while(|pc| {
                        if let crate::document::ParagraphContent::Run(r) = pc {
                            let mut v = true;
                            for rc in r.content.iter() {
                                if let crate::document::RunContent::FieldChar(fc) = rc {
                                    if let Some(ct) = &fc.ty {
                                        if let crate::document::CharType::Begin = ct {
                                            v =false;
                                            break;
                                        }
                                    }
                                };
                            };
                            v
                        } else {
                            true
                        }
                    });

                    if let Some(pc) = iter.next() {

                    }
                } else {
                    // if it's close part, add merged content. Otherwise, ignore it.
                }
            },
            crate::document::BodyContent::Table(_t) => {
                
            }
        }
    }
    let f = docx.write_file(path)?;
    Ok(())
}
