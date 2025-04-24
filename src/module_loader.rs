use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

use crate::{
    error::ParserError,
    parser::YangParser,
    resolver::ReferenceResolver,
    yang::{Import, Module, ReferenceNodes, Submodule, YangFile},
};

/// Internal struct that handles loading, importing and including YANG modules and their dependencies.
pub struct ModuleLoader {
    // Track imported modules by their names.
    imported_modules: HashMap<String, ReferenceNodes>,
    // Map from prefix to module name.
    prefix_to_module: HashMap<String, String>,
}

impl ModuleLoader {
    pub fn new() -> Self {
        Self {
            imported_modules: HashMap::new(),
            prefix_to_module: HashMap::new(),
        }
    }

    /// Load a YANG file from the given path, processing all imports and includes.
    pub fn load_file<P: AsRef<Path>>(mut self, path: P) -> Result<YangFile, ParserError> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).map_err(ParserError::InvalidFile)?;

        // Create a new YangParser and parse the initial module.
        let mut parser = YangParser::new();
        let mut result = parser.parse(&content)?;

        // The entrypoint for parsing should always be a module, not a submodule.
        let module = match &mut result {
            YangFile::Module(module) => module,
            YangFile::Submodule(_) => return Err(ParserError::InvalidParserEntrypoint),
        };

        // Process all included submodules and add their nodes to the main module.
        self.process_includes(path, module, &mut parser)?;

        // Collect imports from the parser, parse them and merge their reference nodes.
        let imports = parser.imports;
        self.process_imports(path, &module.name, imports)?;

        // Create resolver with all reference information (local and imported)
        let resolver = ReferenceResolver::new(parser.reference_nodes, self.imported_modules, self.prefix_to_module);

        // Walk the entire tree and resolve any references.
        resolver.resolve_references(module);

        Ok(result)
    }

    /// Recursively process includes found in the main module and any nested includes.
    fn process_includes<P: AsRef<Path>>(
        &mut self,
        base_path: P,
        module: &mut Module,
        parser: &mut YangParser,
    ) -> Result<(), ParserError> {
        // Submodules will be recursively parsed, so we clone and clear the current list of includes.
        let includes = parser.take_includes();

        for include in includes {
            let parent_dir = base_path.as_ref().parent().unwrap_or_else(|| Path::new("."));
            let submodule_path = parent_dir.join(format!("{}.yang", include.module));
            let submodule_content = fs::read_to_string(&submodule_path).map_err(ParserError::InvalidFile)?;
            let yangfile = parser.parse(&submodule_content)?;

            if let YangFile::Submodule(submodule) = yangfile {
                // Recursively process any includes in this submodule.
                self.process_includes(&submodule_path, module, parser)?;

                // After processing nested includes, merge the submodule's nodes into the main module.
                self.merge_submodule_into_module(&submodule, module);
            } else {
                // This should never happen as included files should always be submodules.
                return Err(ParserError::InvalidInclude(
                    submodule_path.to_string_lossy().into_owned(),
                ));
            }
        }

        Ok(())
    }

    /// Merge a submodule's content into the main module
    fn merge_submodule_into_module(&self, submodule: &Submodule, module: &mut Module) {
        // Merge body nodes from submodule into the main module
        for node in &submodule.body {
            module.body.push(node.clone());
        }

        // Merge revisions that don't already exist in the main module
        for revision in &submodule.revisions {
            if !module.revisions.iter().any(|r| r.date == revision.date) {
                module.revisions.push(revision.clone());
            }
        }
    }

    /// Recursively process imports found in the main module and its included submodules
    fn process_imports<P: AsRef<Path>>(
        &mut self,
        base_path: P,
        current_module: &str,
        initial_imports: Vec<Import>,
    ) -> Result<(), ParserError> {
        let mut imports_to_process = initial_imports;

        // Track processed modules to avoid parsing the same module twice.
        let mut processed_modules = HashSet::new();
        processed_modules.insert(current_module.to_string());

        while !imports_to_process.is_empty() {
            // Probably not optimal, but I don't think it matters that much here.
            let import = imports_to_process.remove(0);

            // Skip if we've already processed this module.
            if self.imported_modules.contains_key(&import.module) || processed_modules.contains(&import.module) {
                // Just update the prefix mapping to map the new prefix to existing module.
                self.prefix_to_module
                    .insert(import.prefix.clone(), import.module.clone());
                continue;
            }

            // Mark this module as processed
            processed_modules.insert(import.module.clone());

            let parent_dir = base_path.as_ref().parent().unwrap_or_else(|| Path::new("."));
            let module_path = parent_dir.join(format!("{}.yang", import.module));

            // Setup new YangParser for the imported module and parse it fully.
            let module_content = fs::read_to_string(&module_path).map_err(ParserError::InvalidFile)?;
            let mut module_parser = YangParser::new();
            let yangfile = module_parser.parse(&module_content)?;

            match yangfile {
                YangFile::Module(mut module) => {
                    // First, process includes in this module to make sure all submodule content is merged.
                    self.process_includes(&module_path, &mut module, &mut module_parser)?;

                    // Store the prefix mapping.
                    self.prefix_to_module
                        .insert(import.prefix.clone(), import.module.clone());

                    // Store the imported module's reference nodes.
                    self.imported_modules
                        .insert(import.module.clone(), module_parser.reference_nodes);

                    // Add any nested imports to our processing queue.
                    for nested_import in module_parser.imports {
                        imports_to_process.push(nested_import);
                    }
                }
                YangFile::Submodule(_) => {
                    // This should never happen as imported files should always be modules
                    return Err(ParserError::InvalidImport(module_path.to_string_lossy().into_owned()));
                }
            }
        }

        Ok(())
    }
}
