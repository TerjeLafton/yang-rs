#![allow(dead_code)]

mod error;
mod module_loader;
mod parser;
mod parser_internal;
mod resolver;
pub mod yang;

// Re-export only what users need
pub use error::ParserError;

/// Parse a YANG module from a file
///
/// This is the main entry point for the YANG parser. It reads the YANG file
/// from the given path, parses it, resolves all imports and references,
/// and returns the parsed module.
///
/// # Arguments
///
/// * `path` - Path to the YANG file to parse
///
/// # Returns
///
/// The parsed YANG file (either a Module or Submodule)
///
/// # Errors
///
/// Returns a ParserError if the file cannot be read or parsed
pub fn parse<P: AsRef<std::path::Path>>(path: P) -> Result<yang::YangFile, ParserError> {
    // Hide implementation details from users
    module_loader::ModuleLoader::new().load_file(path)
}
