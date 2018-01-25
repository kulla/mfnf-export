//! Defines the target trait.

use std::io;
use mediawiki_parser::ast::Element;
use std::collections::HashMap;
use settings::Settings;


/// Marks an exportable target type.
pub trait Target {
    /// export the the ast to `out`.
    fn export(&self,
              root: &Element,
              settings: &Settings,
              out: &mut io::Write) -> io::Result<()>;
    /// get the name of this target.
    fn get_name(&self) -> &str;
    /// does this target operate on the input tree directly or with
    /// mfnf transformations applied?
    fn do_include_sections(&self) -> bool { false }
    /// are make dependencies generated for this target?
    fn do_generate_dependencies(&self) -> bool { false }
    /// extension of the resulting file. Used for make dependency generation.
    fn get_target_extension(&self) -> &str;
    /// mapping of external file extensions to target extensions.
    /// this is useful if external dependencies should be processed by
    /// make for this target.
    fn get_extension_mapping(&self) -> &HashMap<String, String>;
}

