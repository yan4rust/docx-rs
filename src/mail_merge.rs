use std::{collections::HashMap, path::Path};

use crate::{Docx, DocxResult};

pub fn mail_merge<P>(template: &Docx, map: HashMap<String, String>, path: P) -> DocxResult<()>
where
    P: AsRef<Path>,
{
    let mut docx = template.clone();
    for p in template.document.body.content.iter() {
        match p {
            crate::document::BodyContent::Paragraph(p) => todo!(),
            crate::document::BodyContent::Table(t) => {
                docx.document.push(p.clone());
            }
        }
    }
    let f = docx.write_file(path)?;
    Ok(())
}
