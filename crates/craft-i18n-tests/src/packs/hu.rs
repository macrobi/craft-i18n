//! Narrow, recorded repairs applied to a verified pack build, never to its source cache.
use super::{Entry, Kind};
const SOURCE: &str = "d253b8c6188cd0f1b2340c31c8202bf1978611385cf69b211b56991321ccbee0";
pub(super) const NOTICE: &str = "Hungarian Huhyphn pack modifications\nAuthors: Bence Nagy and László Németh. Selected licence: MPL-1.1.\nThe verified LibreOffice/dictionaries 4f1df64b2e8e14751aa7c2205d170ca016071315 source is unchanged in the download cache.\nThe built nonstandard pattern file corrects two malformed replacement declarations:\n.mas5száz8s3zok1ni/sz=3,1,1,14 -> .mas5száz8s3zok1ni/sz=,3,1\ngic5csetek/cs=,3,1 ,1,9 -> gic5csetek/cs=,3,1\nThe standard hyph-hu.tex file is retained for older consumers.\n";
pub(super) fn applies(e: &Entry) -> bool {
    e.locale == "hu" && e.kind == Kind::HyphenationNonstandard && e.sha256 == SOURCE
}
pub(super) fn correct(text: &str) -> Result<String, String> {
    let mut out = text.to_string();
    for (old, new) in [
        (
            ".mas5száz8s3zok1ni/sz=3,1,1,14",
            ".mas5száz8s3zok1ni/sz=,3,1",
        ),
        ("gic5csetek/cs=,3,1 ,1,9", "gic5csetek/cs=,3,1"),
    ] {
        if out.lines().filter(|l| *l == old).count() != 1 {
            return Err("pinned Hungarian replacement line is missing or duplicated".into());
        }
        out = out
            .lines()
            .map(|l| if l == old { new } else { l })
            .collect::<Vec<_>>()
            .join("\n");
        if text.ends_with('\n') {
            out.push('\n');
        }
    }
    Ok(out)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repairs_are_exact_and_marked() {
        let raw = "UTF-8\n.mas5száz8s3zok1ni/sz=3,1,1,14\ngic5csetek/cs=,3,1 ,1,9\n";
        assert_eq!(
            correct(raw).unwrap(),
            "UTF-8\n.mas5száz8s3zok1ni/sz=,3,1\ngic5csetek/cs=,3,1\n"
        );
        assert!(correct("").is_err());
        assert!(correct(&(raw.to_string() + raw)).is_err());
        assert!(NOTICE.contains("MPL-1.1"));
        assert!(NOTICE.contains("Bence Nagy"));
    }
}
