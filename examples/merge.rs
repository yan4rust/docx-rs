use std::path::PathBuf;

use clap::Parser;
use docx_rust::{DocxFile, DocxResult};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// doc1
    #[arg(long)]
    pub doc1: PathBuf,

    /// doc2
    #[arg(long)]
    pub doc2: PathBuf,
}



fn main() -> DocxResult<()> {
    let cli = Cli::parse();
    let fdoc = DocxFile::from_file(&cli.doc1).unwrap();
    // merge.exe --doc1 E:\temp\report_datas\docx_merge\doc1.docx --doc2 E:\temp\report_datas\docx_merge\doc2.docx
    // when parse docx generated by poi-tl,got the following error: 
    // thread 'main' panicked at examples\merge.rs:22:33:
    // called `Result::unwrap()` on an `Err` value: Xml(MissingField { name: "AbstractNum", field: "nsid" })
    let mut docx = fdoc.parse().unwrap();

    let fdoc2 = DocxFile::from_file(&cli.doc2).unwrap();
    let docx2 = fdoc2.parse().unwrap();
    for content in docx2.document.body.content {
        docx.document.push(content);
    }

    docx.write_file("out.docx").unwrap();

    Ok(())

}
