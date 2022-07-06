extern crate docx;
use docx::DocxFile;

#[test]
fn read_and_relace() {
    // reader
    let path = std::path::Path::new("./tests/aaa/aa.docx");
    let book = DocxFile::from_file(path).unwrap();
    let mut docx = book.parse().unwrap();
    docx.document.body.replace_text_simple("好日子", "好天气");

    let path2 = std::path::Path::new("./tests/bbb/aa.docx");
    let _d = docx.write_file(path2).unwrap();
}
