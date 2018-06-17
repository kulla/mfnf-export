use preamble::*;

use std::io;

/// serialize to html
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(default)]
pub struct HTMLTarget {}

impl Target for HTMLTarget {
    fn include_sections(&self) -> bool {
        true
    }
    fn target_extension(&self) -> &str {
        "html"
    }
    fn extension_for(&self, ext: &str) -> &str {
        "%"
    }

    fn export<'a>(
        &self,
        root: &'a Element,
        settings: &Settings,
        _: &[String],
        out: &mut io::Write,
    ) -> io::Result<()> {
        print!("Hello World!");
        Ok(())
    }
}
