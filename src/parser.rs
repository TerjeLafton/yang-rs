use std::{fs, path::Path};

use pest::{iterators::Pair, Parser};

use crate::{error::ParserError, ir::*, Rule, YangModule};

pub struct YangParser {
    // imports and includes are stored to be processed after the original module is done being parsed.
    imports: Vec<Import>,
    includes: Vec<Include>,

    // reference_nodes is used to store the nodes which are not part of the data-tree and might be referenced
    // by other nodes in the data-tree.
    reference_nodes: ReferenceNodes,

    // Properties used during parsing.
    // current_path is used to track the path as we walk the AST and have to store nodes with their full path
    // in the reference_nodes struct.
    current_path: String,

    // current_belongs_to_prefix is used when parsing submodules to track which prefix they use for references
    // to nodes in the module they belong to. This prefix is stripped away as the nodes will be merged with the
    // original modules nodes anyway.
    current_belongs_to_prefix: Option<String>,
}

impl YangParser {
    fn new() -> Self {
        Self {
            reference_nodes: ReferenceNodes::default(),
            imports: Vec::new(),
            includes: Vec::new(),
            current_path: String::from("/"),
            current_belongs_to_prefix: None,
        }
    }

    // parse_file is the main API for the YangParser. It starts out with a single module, which will be the
    // entrypoint for the parsing. It will parse any references YANG file and resolve references between them.
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<YangFile, ParserError> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).map_err(|err| ParserError::InvalidFile(err))?;

        let mut parser = Self::new();
        let mut result = parser.parse(&content)?;

        // The entrypoint for parsing should always be a module, not a submodule. If it is a submodule, we stop
        // parsing and return an error to the user. They most likely selected the wrong file.
        let module = match &mut result {
            YangFile::Module(module) => module,
            YangFile::Submodule(_) => return Err(ParserError::InvalidParserEntrypoint),
        };

        // Process all included submodules and add their nodes to the main module
        parser.process_includes(path, module)?;

        // let resolver = ReferenceResolver::new(parser.reference_nodes.groupings);
        // resolver.resolve_references(module);

        Ok(result)
    }

    // Recursively process includes found in the main module and any includes found nested.
    fn process_includes<P: AsRef<Path>>(&mut self, base_path: P, module: &mut Module) -> Result<(), ParserError> {
        // We will recursively parse submodules, so we clone and clear the current list of includes.
        let includes = self.includes.clone();
        self.includes.clear();

        for include in includes {
            // Get the directory part of the base path.
            let parent_dir = base_path.as_ref().parent().unwrap_or_else(|| Path::new("."));

            // Construct the path to the included submodule with .yang extension.
            let submodule_path = parent_dir.join(format!("{}.yang", include.module));

            // Read and parse the submodule
            let submodule_content = fs::read_to_string(&submodule_path).map_err(|err| ParserError::InvalidFile(err))?;
            let yangfile = self.parse(&submodule_content)?;

            if let YangFile::Submodule(submodule) = yangfile {
                // Recursively process any includes in this submodule.
                self.process_includes(&submodule_path, module)?;

                // After processing nested includes, merge the submodule's nodes into the main module.
                for node in submodule.body {
                    module.body.push(node);
                }

                for revision in submodule.revisions {
                    if !module.revisions.iter().any(|r| r.date == revision.date) {
                        module.revisions.push(revision);
                    }
                }
            } else {
                // This should never happen as included files should always be submodules.
                // If so, we return an error saying which module failed.
                return Err(ParserError::InvalidInclude(
                    submodule_path.to_string_lossy().into_owned(),
                ));
            }
        }

        Ok(())
    }

    // parse is the entrypoint for the actual parsing for the crate. It starts off with assuming that the file
    // is a valid YANG file, as per the Pest grammar. It then starts off the chain of parsing functions and
    // works itself through the entire tree.
    fn parse(&mut self, input: &str) -> Result<YangFile, ParserError> {
        let module = YangModule::parse(Rule::file, input)
            .map_err(|err| ParserError::ParseError(err))?
            .next()
            .expect("a yang file to always include a module");

        match module.as_rule() {
            Rule::module => Ok(YangFile::Module(self.parse_module(module))),
            Rule::submodule => Ok(YangFile::Submodule(self.parse_submodule(module))),
            _ => unreachable!("parsing a file can only result in a module or submodule"),
        }
    }

    fn parse_module(&mut self, input: Pair<Rule>) -> Module {
        let mut module = Module::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => module.name = self.parse_string(child),
                Rule::prefix => module.prefix = self.parse_string(child),
                Rule::namespace => module.namespace = self.parse_string(child),
                Rule::yang_version => module.yang_version = Some(self.parse_string(child)),
                Rule::organization => module.meta.organization = Some(self.parse_string(child)),
                Rule::contact => module.meta.contact = Some(self.parse_string(child)),
                Rule::description => module.meta.description = Some(self.parse_string(child)),
                Rule::reference => module.meta.reference = Some(self.parse_string(child)),
                Rule::revision => module.revisions.push(self.parse_revision(child)),
                Rule::import => self.parse_import(child),
                Rule::include => self.parse_include(child),

                // parse_body returns an option based on if the node it parsed was a data node or not.
                // Data nodes return Some(node) while other nodes return None.
                Rule::body => match self.parse_body(child) {
                    Some(node) => module.body.push(node),
                    None => {}
                },
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        module
    }

    fn parse_submodule(&mut self, input: Pair<Rule>) -> Submodule {
        let mut submodule = Submodule::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => submodule.name = self.parse_string(child),
                Rule::belongs_to => {
                    submodule.belongs_to = self.parse_belongs_to(child);
                    // Store the prefix from the belongs-to statement for when parsing uses statements later.
                    // See parse_uses function for more details.
                    self.current_belongs_to_prefix = Some(submodule.belongs_to.prefix.clone());
                }
                Rule::yang_version => submodule.yang_version = Some(self.parse_string(child)),
                Rule::organization => submodule.meta.organization = Some(self.parse_string(child)),
                Rule::contact => submodule.meta.contact = Some(self.parse_string(child)),
                Rule::description => submodule.meta.description = Some(self.parse_string(child)),
                Rule::reference => submodule.meta.reference = Some(self.parse_string(child)),
                Rule::revision => submodule.revisions.push(self.parse_revision(child)),
                Rule::import => self.parse_import(child),
                Rule::include => self.parse_include(child),
                Rule::body => match self.parse_body(child) {
                    Some(node) => submodule.body.push(node),
                    None => {}
                },
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        // Clear the belongs-to prefix after parsing the submodule
        self.current_belongs_to_prefix = None;

        submodule
    }

    // parse_body is a bit different than most parse functions as it might not always return the node is just parsed.
    // This is because a SchemaNode rerpresents both data nodes like containers and leaves, but also nodes like
    // groupings and typedefs, which are only referenced by other data nodes.
    // Therefor data nodes are returned by the function while other nodes are instead stored either for later
    // processing or for resolving references later.
    fn parse_body(&mut self, input: Pair<Rule>) -> Option<SchemaNode> {
        let node = input.into_inner().next().expect("to always have inner nodes");

        match node.as_rule() {
            Rule::data_def => Some(SchemaNode::DataDef(self.parse_data_def(node))),
            Rule::rpc => Some(SchemaNode::Rpc(self.parse_rpc(node))),
            Rule::notification => Some(SchemaNode::Notification(self.parse_notification(node))),
            Rule::extension => {
                self.parse_extension(node);
                None
            }
            Rule::feature => {
                self.parse_feature(node);
                None
            }
            Rule::identity => {
                self.parse_identity(node);
                None
            }
            Rule::type_def => {
                self.parse_type_def(node);
                None
            }
            Rule::augment => {
                self.parse_augment(node);
                None
            }
            Rule::deviation => {
                self.parse_deviation(node);
                None
            }
            Rule::grouping => {
                self.parse_grouping(node);
                None
            }
            _ => unreachable!("Unexpected rule: {:?}", node.as_rule()),
        }
    }

    fn parse_anydata(&mut self, input: Pair<Rule>) -> Anydata {
        let mut anydata = Anydata::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => anydata.name = self.parse_string(child),
                Rule::when => anydata.when = Some(self.parse_when(child)),
                Rule::if_feature => anydata.if_features.push(self.parse_string(child)),
                Rule::must => anydata.must.push(self.parse_must(child)),
                Rule::config => anydata.config = Some(self.parse_boolean(child)),
                Rule::mandatory => anydata.mandatory = Some(self.parse_boolean(child)),
                Rule::status => anydata.status = Some(self.parse_status(child)),
                Rule::description => anydata.description = Some(self.parse_string(child)),
                Rule::reference => anydata.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        anydata
    }

    fn parse_anyxml(&mut self, input: Pair<Rule>) -> Anyxml {
        let mut anyxml = Anyxml::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => anyxml.name = self.parse_string(child),
                Rule::when => anyxml.when = Some(self.parse_when(child)),
                Rule::if_feature => anyxml.if_features.push(self.parse_string(child)),
                Rule::must => anyxml.must.push(self.parse_must(child)),
                Rule::config => anyxml.config = Some(self.parse_boolean(child)),
                Rule::mandatory => anyxml.mandatory = Some(self.parse_boolean(child)),
                Rule::status => anyxml.status = Some(self.parse_status(child)),
                Rule::description => anyxml.description = Some(self.parse_string(child)),
                Rule::reference => anyxml.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        anyxml
    }

    fn parse_deviation(&mut self, input: Pair<Rule>) {
        let mut deviation = Deviation::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => deviation.target = self.parse_string(child),
                Rule::description => deviation.description = Some(self.parse_string(child)),
                Rule::reference => deviation.reference = Some(self.parse_string(child)),
                Rule::deviation_not_supported => deviation.not_supported = true,
                Rule::deviate_add => deviation.add.push(self.parse_deviate_add(child)),
                Rule::deviate_delete => deviation.delete.push(self.parse_deviate_delete(child)),
                Rule::deviate_replace => deviation.replace.push(self.parse_deviate_replace(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        self.reference_nodes.deviations.push(deviation);
    }

    fn parse_deviate_add(&mut self, input: Pair<Rule>) -> DeviateAdd {
        let mut deviate = DeviateAdd::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::units => deviate.units = Some(self.parse_string(child)),
                Rule::must => deviate.must.push(self.parse_must(child)),
                Rule::unique => deviate.unique.push(self.parse_string(child)),
                Rule::default => deviate.default.push(self.parse_string(child)),
                Rule::config => deviate.config = Some(self.parse_boolean(child)),
                Rule::mandatory => deviate.mandatory = Some(self.parse_boolean(child)),
                Rule::min_elements => deviate.min_elements = Some(self.parse_integer(child)),
                Rule::max_elements => deviate.max_elements = Some(self.parse_max_elements(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        deviate
    }

    fn parse_deviate_delete(&mut self, input: Pair<Rule>) -> DeviateDelete {
        let mut deviate = DeviateDelete::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::units => deviate.units = Some(self.parse_string(child)),
                Rule::default => deviate.default.push(self.parse_string(child)),
                Rule::must => deviate.must.push(self.parse_must(child)),
                Rule::unique => deviate.unique.push(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        deviate
    }

    fn parse_deviate_replace(&mut self, input: Pair<Rule>) -> DeviateReplace {
        let mut deviate = DeviateReplace::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::type_info => deviate.type_info = Some(self.parse_type_info(child)),
                Rule::units => deviate.units = Some(self.parse_string(child)),
                Rule::default => deviate.default.push(self.parse_string(child)),
                Rule::config => deviate.config = Some(self.parse_boolean(child)),
                Rule::mandatory => deviate.mandatory = Some(self.parse_boolean(child)),
                Rule::min_elements => deviate.min_elements = Some(self.parse_integer(child)),
                Rule::max_elements => deviate.max_elements = Some(self.parse_max_elements(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        deviate
    }

    fn parse_data_def(&mut self, input: Pair<Rule>) -> DataDef {
        let node = input.into_inner().next().expect("to always have inner nodes");

        match node.as_rule() {
            Rule::container => DataDef::Container(self.parse_container(node)),
            Rule::leaf => DataDef::Leaf(self.parse_leaf(node)),
            Rule::leaf_list => DataDef::LeafList(self.parse_leaf_list(node)),
            Rule::list => DataDef::List(self.parse_list(node)),
            Rule::choice => DataDef::Choice(self.parse_choice(node)),
            Rule::anydata => DataDef::AnyData(self.parse_anydata(node)),
            Rule::anyxml => DataDef::Anyxml(self.parse_anyxml(node)),
            Rule::uses => DataDef::Uses(self.parse_uses(node)),
            _ => unreachable!("Unexpected rule: {:?}", node.as_rule()),
        }
    }

    fn parse_container(&mut self, input: Pair<Rule>) -> Container {
        let mut container = Container::default();
        let mut input = input.into_inner();
        let name = self.parse_string(input.next().expect("first child to always be the name"));
        container.name = name.clone();

        self.with_path_scope(name, |this| {
            for child in input {
                match child.as_rule() {
                    Rule::when => container.when = Some(this.parse_when(child)),
                    Rule::if_feature => container.if_features.push(this.parse_string(child)),
                    Rule::must => container.must.push(this.parse_must(child)),
                    Rule::presence => container.presence = Some(this.parse_string(child)),
                    Rule::config => container.config = Some(this.parse_boolean(child)),
                    Rule::status => container.status = Some(this.parse_status(child)),
                    Rule::description => container.description = Some(this.parse_string(child)),
                    Rule::reference => container.reference = Some(this.parse_string(child)),
                    Rule::type_def => this.parse_type_def(child),
                    Rule::grouping => this.parse_grouping(child),
                    Rule::data_def => container.data_defs.push(this.parse_data_def(child)),
                    Rule::action => container.actions.push(this.parse_action(child)),
                    Rule::notification => container.notifications.push(this.parse_notification(child)),
                    _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
                }
            }
        });

        container
    }

    fn parse_choice(&mut self, input: Pair<Rule>) -> Choice {
        let mut choice = Choice::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => choice.name = self.parse_string(child),
                Rule::when => choice.when = Some(self.parse_when(child)),
                Rule::if_feature => choice.if_features.push(self.parse_string(child)),
                Rule::default => choice.default = Some(self.parse_string(child)),
                Rule::config => choice.config = Some(self.parse_boolean(child)),
                Rule::mandatory => choice.mandatory = Some(self.parse_boolean(child)),
                Rule::status => choice.status = Some(self.parse_status(child)),
                Rule::description => choice.description = Some(self.parse_string(child)),
                Rule::reference => choice.reference = Some(self.parse_string(child)),
                Rule::long_case => choice.cases.push(Case::LongCase(self.parse_long_case(child))),
                Rule::short_case => choice.cases.push(Case::ShortCase(self.parse_short_case(child))),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        choice
    }

    fn parse_short_case(&mut self, input: Pair<Rule>) -> ShortCase {
        let node = input.into_inner().next().expect("to always have inner node");

        match node.as_rule() {
            Rule::choice => ShortCase::Choice(self.parse_choice(node)),
            Rule::container => ShortCase::Container(self.parse_container(node)),
            Rule::leaf => ShortCase::Leaf(self.parse_leaf(node)),
            Rule::leaf_list => ShortCase::LeafList(self.parse_leaf_list(node)),
            Rule::list => ShortCase::List(self.parse_list(node)),
            Rule::anydata => ShortCase::Anydata(self.parse_anydata(node)),
            Rule::anyxml => ShortCase::Anyxml(self.parse_anyxml(node)),
            _ => unreachable!("Unexpected rule: {:?}", node.as_rule()),
        }
    }

    fn parse_long_case(&mut self, input: Pair<Rule>) -> LongCase {
        let mut case = LongCase::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => case.name = self.parse_string(child),
                Rule::when => case.when = Some(self.parse_when(child)),
                Rule::if_feature => case.if_features.push(self.parse_string(child)),
                Rule::status => case.status = Some(self.parse_status(child)),
                Rule::description => case.description = Some(self.parse_string(child)),
                Rule::reference => case.reference = Some(self.parse_string(child)),
                Rule::data_def => case.data_defs.push(self.parse_data_def(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        case
    }

    fn parse_uses(&mut self, input: Pair<Rule>) -> Uses {
        let mut uses = Uses::default();

        for child in input.into_inner() {
            match child.as_rule() {
                // Uses statements primary function is to reference groupings to add nodes to the data tree.
                // When a uses statement in a submodule is referencing a grouping statement in the main module,
                // it adds a prefix according to the submodules belongs-to statement.
                // This prefix is stripped away as it is intended for the readers of the YANG file and not needed
                // to resolve the reference. A submodules nodes are merged with the nodes of it's main module and thus
                // a reference to that main module would not need a prefix.
                Rule::string => {
                    let mut grouping_name = self.parse_string(child);

                    // Check if the parser has a belongs-to prefix set. This only happens when the parser starts
                    // parsing a submodule and is removed again once parsing of said submodule is done.
                    if let Some(prefix) = &self.current_belongs_to_prefix {
                        // Check if the grouping name has the prefix from the belongs-to statement of the submodule.
                        let prefixed_name = format!("{}:", prefix);
                        if grouping_name.starts_with(&prefixed_name) {
                            // Remove the prefix from the reference as it is not needed.
                            grouping_name = grouping_name[prefixed_name.len()..].to_string();
                        }
                    }

                    uses.grouping = grouping_name;
                }
                Rule::when => uses.when = Some(self.parse_when(child)),
                Rule::if_feature => uses.if_features.push(self.parse_string(child)),
                Rule::status => uses.status = Some(self.parse_status(child)),
                Rule::description => uses.description = Some(self.parse_string(child)),
                Rule::reference => uses.reference = Some(self.parse_string(child)),
                Rule::refine => uses.refines.push(self.parse_refine(child)),
                Rule::augment => self.parse_augment(child),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        uses
    }

    fn parse_augment(&mut self, input: Pair<Rule>) {
        let mut augment = Augment::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => augment.target = self.parse_string(child),
                Rule::when => augment.when = Some(self.parse_when(child)),
                Rule::if_feature => augment.if_features.push(self.parse_string(child)),
                Rule::status => augment.status = Some(self.parse_status(child)),
                Rule::description => augment.description = Some(self.parse_string(child)),
                Rule::reference => augment.reference = Some(self.parse_string(child)),
                Rule::data_def => augment.data_defs.push(self.parse_data_def(child)),
                Rule::long_case => augment.cases.push(Case::LongCase(self.parse_long_case(child))),
                Rule::action => augment.actions.push(self.parse_action(child)),
                Rule::notification => augment.notifications.push(self.parse_notification(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        self.reference_nodes.augments.push(augment);
    }

    fn parse_refine(&mut self, input: Pair<Rule>) -> Refine {
        let mut refine = Refine::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => refine.target = self.parse_string(child),
                Rule::if_feature => refine.if_features.push(self.parse_string(child)),
                Rule::must => refine.must.push(self.parse_must(child)),
                Rule::presence => refine.presence = Some(self.parse_string(child)),
                Rule::default => refine.default.push(self.parse_string(child)),
                Rule::config => refine.config = Some(self.parse_boolean(child)),
                Rule::mandatory => refine.mandatory = Some(self.parse_boolean(child)),
                Rule::min_elements => refine.min_elements = Some(self.parse_integer(child)),
                Rule::max_elements => refine.max_elements = Some(self.parse_max_elements(child)),
                Rule::description => refine.description = Some(self.parse_string(child)),
                Rule::reference => refine.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        refine
    }

    fn parse_output(&mut self, input: Pair<Rule>) -> Output {
        let mut output = Output::default();

        self.with_path_scope("output".into(), |this| {
            for child in input.into_inner() {
                match child.as_rule() {
                    Rule::must => output.must.push(this.parse_must(child)),
                    Rule::type_def => this.parse_type_def(child),
                    Rule::grouping => this.parse_grouping(child),
                    Rule::data_def => output.data_defs.push(this.parse_data_def(child)),
                    _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
                }
            }
        });

        output
    }

    fn parse_input(&mut self, input: Pair<Rule>) -> Input {
        let mut new_input = Input::default();

        self.with_path_scope("input".into(), |this| {
            for child in input.into_inner() {
                match child.as_rule() {
                    Rule::must => new_input.must.push(this.parse_must(child)),
                    Rule::type_def => this.parse_type_def(child),
                    Rule::grouping => this.parse_grouping(child),
                    Rule::data_def => new_input.data_defs.push(this.parse_data_def(child)),
                    _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
                }
            }
        });

        new_input
    }

    fn parse_rpc(&mut self, input: Pair<Rule>) -> Rpc {
        let mut rpc = Rpc::default();
        let mut input = input.into_inner();
        let name = self.parse_string(input.next().expect("first child to always be the name"));
        rpc.name = name.clone();

        self.with_path_scope(name, |this| {
            for child in input {
                match child.as_rule() {
                    Rule::input => rpc.input = Some(this.parse_input(child)),
                    Rule::output => rpc.output = Some(this.parse_output(child)),
                    Rule::if_feature => rpc.if_features.push(this.parse_string(child)),
                    Rule::must => rpc.must.push(this.parse_must(child)),
                    Rule::status => rpc.status = Some(this.parse_status(child)),
                    Rule::description => rpc.description = Some(this.parse_string(child)),
                    Rule::reference => rpc.reference = Some(this.parse_string(child)),
                    Rule::type_def => this.parse_type_def(child),
                    Rule::grouping => this.parse_grouping(child),
                    _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
                }
            }
        });

        rpc
    }

    fn parse_action(&mut self, input: Pair<Rule>) -> Action {
        let mut action = Action::default();
        let mut input = input.into_inner();
        let name = self.parse_string(input.next().expect("first child to always be the name"));
        action.name = name.clone();

        self.with_path_scope(name, |this| {
            for child in input {
                match child.as_rule() {
                    Rule::input => action.input = Some(this.parse_input(child)),
                    Rule::output => action.output = Some(this.parse_output(child)),
                    Rule::if_feature => action.if_features.push(this.parse_string(child)),
                    Rule::must => action.must.push(this.parse_must(child)),
                    Rule::status => action.status = Some(this.parse_status(child)),
                    Rule::description => action.description = Some(this.parse_string(child)),
                    Rule::reference => action.reference = Some(this.parse_string(child)),
                    Rule::type_def => this.parse_type_def(child),
                    Rule::grouping => this.parse_grouping(child),
                    _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
                }
            }
        });

        action
    }

    fn parse_notification(&mut self, input: Pair<Rule>) -> Notification {
        let mut notification = Notification::default();
        let mut input = input.into_inner();
        let name = self.parse_string(input.next().expect("first child to always be the name"));
        notification.name = name.clone();

        self.with_path_scope(name, |this| {
            for child in input {
                match child.as_rule() {
                    Rule::data_def => notification.data_defs.push(this.parse_data_def(child)),
                    Rule::if_feature => notification.if_features.push(this.parse_string(child)),
                    Rule::must => notification.must.push(this.parse_must(child)),
                    Rule::status => notification.status = Some(this.parse_status(child)),
                    Rule::description => notification.description = Some(this.parse_string(child)),
                    Rule::reference => notification.reference = Some(this.parse_string(child)),
                    Rule::type_def => this.parse_type_def(child),
                    Rule::grouping => this.parse_grouping(child),
                    _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
                }
            }
        });

        notification
    }

    fn parse_feature(&mut self, input: Pair<Rule>) {
        let mut feature = Feature::default();
        let mut input = input.into_inner();
        let name = self.parse_string(input.next().expect("first child to always be the name"));
        feature.name = name.clone();

        for child in input {
            match child.as_rule() {
                Rule::if_feature => feature.if_features.push(self.parse_string(child)),
                Rule::status => feature.status = Some(self.parse_status(child)),
                Rule::description => feature.description = Some(self.parse_string(child)),
                Rule::reference => feature.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        let path = format!("{}{}", self.current_path, name);
        self.reference_nodes.features.insert(path, feature);
    }

    fn parse_extension(&mut self, input: Pair<Rule>) {
        let mut extension = Extension::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => extension.name = self.parse_string(child),
                Rule::argument => extension.argument = Some(self.parse_argument(child)),
                Rule::status => extension.status = Some(self.parse_status(child)),
                Rule::description => extension.description = Some(self.parse_string(child)),
                Rule::reference => extension.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        self.reference_nodes.extensions.push(extension);
    }

    fn parse_when(&mut self, input: Pair<Rule>) -> When {
        let mut when = When::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => when.condition = self.parse_string(child),
                Rule::description => when.description = Some(self.parse_string(child)),
                Rule::reference => when.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        when
    }

    fn parse_belongs_to(&mut self, input: Pair<Rule>) -> BelongsTo {
        let mut belongs_to = BelongsTo::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => belongs_to.module = self.parse_string(child),
                Rule::prefix => belongs_to.prefix = self.parse_string(child),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        belongs_to
    }

    fn parse_argument(&mut self, input: Pair<Rule>) -> Argument {
        let mut argument = Argument::default();
        let mut input = input.into_inner();

        argument.name = self.parse_string(input.next().expect("first child to always be the name"));
        if let Some(yin_element) = input.next() {
            argument.yin_element = Some(self.parse_boolean(yin_element))
        }

        argument
    }

    fn parse_grouping(&mut self, input: Pair<Rule>) {
        let mut grouping = Grouping::default();
        let mut input = input.into_inner();
        let name = self.parse_string(input.next().expect("first child to always be the name"));
        grouping.name = name.clone();

        self.with_path_scope(name, |this| {
            for child in input {
                match child.as_rule() {
                    Rule::status => grouping.status = Some(this.parse_status(child)),
                    Rule::description => grouping.description = Some(this.parse_string(child)),
                    Rule::reference => grouping.reference = Some(this.parse_string(child)),
                    Rule::type_def => this.parse_type_def(child),
                    Rule::grouping => this.parse_grouping(child),
                    Rule::data_def => grouping.data_defs.push(this.parse_data_def(child)),
                    Rule::action => grouping.actions.push(this.parse_action(child)),
                    Rule::notification => grouping.notifications.push(this.parse_notification(child)),
                    _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
                }
            }
        });

        let path = format!("{}{}", self.current_path, grouping.name);
        self.reference_nodes.groupings.insert(path, grouping);
    }

    fn parse_type_def(&mut self, input: Pair<Rule>) {
        let mut type_def = TypeDef::default();
        let mut input = input.into_inner();
        let name = self.parse_string(input.next().expect("first child to always be the name"));
        type_def.name = name.clone();

        for child in input {
            match child.as_rule() {
                Rule::type_info => type_def.type_info = self.parse_type_info(child),
                Rule::units => type_def.units = Some(self.parse_string(child)),
                Rule::default => type_def.default = Some(self.parse_string(child)),
                Rule::status => type_def.status = Some(self.parse_status(child)),
                Rule::description => type_def.description = Some(self.parse_string(child)),
                Rule::reference => type_def.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        let path = format!("{}{}", self.current_path, name);
        self.reference_nodes.type_defs.insert(path, type_def);
    }
    fn parse_leaf_list(&mut self, input: Pair<Rule>) -> LeafList {
        let mut leaf_list = LeafList::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => leaf_list.name = self.parse_string(child),
                Rule::when => leaf_list.when = Some(self.parse_when(child)),
                Rule::if_feature => leaf_list.if_features.push(self.parse_string(child)),
                Rule::type_info => leaf_list.type_info = self.parse_type_info(child),
                Rule::units => leaf_list.units = Some(self.parse_string(child)),
                Rule::must => leaf_list.must.push(self.parse_must(child)),
                Rule::default => leaf_list.default.push(self.parse_string(child)),
                Rule::config => leaf_list.config = Some(self.parse_boolean(child)),
                Rule::ordered_by => leaf_list.ordered_by = Some(self.parse_ordered_by(child)),
                Rule::min_elements => leaf_list.min_elements = Some(self.parse_integer(child)),
                Rule::max_elements => leaf_list.max_elements = Some(self.parse_max_elements(child)),
                Rule::status => leaf_list.status = Some(self.parse_status(child)),
                Rule::description => leaf_list.description = Some(self.parse_string(child)),
                Rule::reference => leaf_list.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        leaf_list
    }

    fn parse_leaf(&mut self, input: Pair<Rule>) -> Leaf {
        let mut leaf = Leaf::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => leaf.name = self.parse_string(child),
                Rule::when => leaf.when = Some(self.parse_when(child)),
                Rule::if_feature => leaf.if_features.push(self.parse_string(child)),
                Rule::type_info => leaf.type_info = self.parse_type_info(child),
                Rule::units => leaf.units = Some(self.parse_string(child)),
                Rule::must => leaf.must.push(self.parse_must(child)),
                Rule::default => leaf.default = Some(self.parse_string(child)),
                Rule::config => leaf.config = Some(self.parse_boolean(child)),
                Rule::mandatory => leaf.mandatory = Some(self.parse_boolean(child)),
                Rule::status => leaf.status = Some(self.parse_status(child)),
                Rule::description => leaf.description = Some(self.parse_string(child)),
                Rule::reference => leaf.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        leaf
    }

    fn parse_list(&mut self, input: Pair<Rule>) -> List {
        let mut list = List::default();
        let mut input = input.into_inner();
        let name = self.parse_string(input.next().expect("first child to always be the name"));
        list.name = name.clone();

        self.with_path_scope(name, |this| {
            for child in input {
                match child.as_rule() {
                    Rule::when => list.when = Some(this.parse_when(child)),
                    Rule::if_feature => list.if_features.push(this.parse_string(child)),
                    Rule::must => list.must.push(this.parse_must(child)),
                    Rule::key => list.key = Some(this.parse_string(child)),
                    Rule::unique => list.unique.push(this.parse_string(child)),
                    Rule::config => list.config = Some(this.parse_boolean(child)),
                    Rule::min_elements => list.min_elements = Some(this.parse_integer(child)),
                    Rule::max_elements => list.max_elements = Some(this.parse_max_elements(child)),
                    Rule::ordered_by => list.ordered_by = Some(this.parse_ordered_by(child)),
                    Rule::status => list.status = Some(this.parse_status(child)),
                    Rule::description => list.description = Some(this.parse_string(child)),
                    Rule::reference => list.reference = Some(this.parse_string(child)),
                    Rule::type_def => this.parse_type_def(child),
                    Rule::grouping => this.parse_grouping(child),
                    Rule::data_def => list.data_defs.push(this.parse_data_def(child)),
                    Rule::action => list.actions.push(this.parse_action(child)),
                    Rule::notification => list.notifications.push(this.parse_notification(child)),
                    _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
                }
            }
        });

        list
    }

    fn parse_type_info(&mut self, input: Pair<Rule>) -> TypeInfo {
        let mut type_info = TypeInfo::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => type_info.name = self.parse_string(child),
                Rule::numberical_restriction => type_info.type_body = Some(self.parse_numerical(child)),
                Rule::decimal64_specification => type_info.type_body = Some(self.parse_decimal(child)),
                Rule::string_restriction => type_info.type_body = Some(self.parse_string_restriction(child)),
                Rule::enum_specification => type_info.type_body = Some(self.parse_enum(child)),
                Rule::leafref_specification => type_info.type_body = Some(self.parse_leafref(child)),
                Rule::identityref_specification => type_info.type_body = Some(self.parse_identityref(child)),
                Rule::bits_specification => type_info.type_body = Some(self.parse_bit_specification(child)),
                Rule::binary_specification => type_info.type_body = Some(self.parse_binary_specification(child)),
                Rule::union_specification => type_info.type_body = Some(self.parse_union_specification(child)),
                Rule::instance_identifier_specification => {
                    type_info.type_body = Some(self.parse_instance_identifier(child))
                }
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        return type_info;
    }

    fn parse_union_specification(&mut self, input: Pair<Rule>) -> TypeBody {
        let mut types = Vec::new();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::type_info => types.push(self.parse_type_info(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        TypeBody::Union { types }
    }

    fn parse_binary_specification(&mut self, input: Pair<Rule>) -> TypeBody {
        match input.into_inner().next() {
            Some(length) => TypeBody::Binary {
                length: Some(self.parse_length(length)),
            },
            None => TypeBody::Binary { length: None },
        }
    }

    fn parse_bit_specification(&mut self, input: Pair<Rule>) -> TypeBody {
        let mut bits = Vec::new();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::bit => bits.push(self.parse_bit(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        TypeBody::Bits { bits }
    }

    fn parse_identity(&mut self, input: Pair<Rule>) {
        let mut identity = Identity::default();
        let mut input = input.into_inner();
        let name = input.next().expect("first child to always be the name");

        for child in input {
            match child.as_rule() {
                Rule::if_feature => identity.if_features.push(self.parse_string(child)),
                Rule::base => identity.bases.push(self.parse_string(child)),
                Rule::status => identity.status = Some(self.parse_status(child)),
                Rule::description => identity.description = Some(self.parse_string(child)),
                Rule::reference => identity.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        let path = format!("{}{}", self.current_path, name);
        self.reference_nodes.identities.insert(path, identity);
    }

    fn parse_identityref(&mut self, input: Pair<Rule>) -> TypeBody {
        let mut bases = Vec::new();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::base => bases.push(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        TypeBody::Identityref { bases }
    }

    fn parse_instance_identifier(&mut self, input: Pair<Rule>) -> TypeBody {
        TypeBody::InstanceIdentifier {
            require_instance: self.parse_boolean(
                input
                    .into_inner()
                    .next()
                    .expect("instance identifier to always have a require instance node"),
            ),
        }
    }

    fn parse_leafref(&mut self, input: Pair<Rule>) -> TypeBody {
        let mut leafref = input.into_inner();
        let path = self.parse_string(leafref.next().expect("first child of leafref to be the path"));

        match leafref.next() {
            Some(require_instance) => TypeBody::Leafref {
                path,
                require_instance: Some(self.parse_boolean(require_instance)),
            },
            None => TypeBody::Leafref {
                path,
                require_instance: None,
            },
        }
    }

    fn parse_string_restriction(&mut self, input: Pair<Rule>) -> TypeBody {
        let mut length = None;
        let mut patterns = Vec::new();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::length => length = Some(self.parse_length(child)),
                Rule::pattern => patterns.push(self.parse_pattern(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        TypeBody::String { length, patterns }
    }

    fn parse_decimal(&mut self, input: Pair<Rule>) -> TypeBody {
        let mut decimal_node = input.into_inner();
        let fractional_digits = self.parse_string(
            decimal_node
                .next()
                .expect("decimal's first child always to be a fractional_digits node"),
        );

        match decimal_node.next() {
            Some(range) => TypeBody::Decimal64 {
                fraction_digits: fractional_digits,
                range: Some(self.parse_range(range)),
            },
            None => TypeBody::Decimal64 {
                fraction_digits: fractional_digits,
                range: None,
            },
        }
    }

    fn parse_numerical(&mut self, input: Pair<Rule>) -> TypeBody {
        TypeBody::Numerical {
            range: self.parse_range(
                input
                    .into_inner()
                    .next()
                    .expect("numerical to always have range as the only child"),
            ),
        }
    }

    fn parse_enum(&mut self, input: Pair<Rule>) -> TypeBody {
        let mut enums = Vec::new();
        for enum_child in input.into_inner() {
            let mut enum_value = EnumValue::default();
            for child in enum_child.into_inner() {
                match child.as_rule() {
                    Rule::string => enum_value.name = self.parse_string(child),
                    Rule::if_feature => enum_value.if_features.push(self.parse_string(child)),
                    Rule::value => enum_value.value = Some(self.parse_integer(child)),
                    Rule::status => enum_value.status = Some(self.parse_status(child)),
                    Rule::description => enum_value.description = Some(self.parse_string(child)),
                    Rule::reference => enum_value.reference = Some(self.parse_string(child)),
                    _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
                }
            }
            enums.push(enum_value);
        }

        TypeBody::Enum { enums }
    }

    fn parse_bit(&mut self, input: Pair<Rule>) -> Bit {
        let mut bit = Bit::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => bit.name = self.parse_string(child),
                Rule::if_feature => bit.if_features.push(self.parse_string(child)),
                Rule::position => bit.position = Some(self.parse_integer(child)),
                Rule::status => bit.status = Some(self.parse_status(child)),
                Rule::description => bit.description = Some(self.parse_string(child)),
                Rule::reference => bit.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        bit
    }

    fn parse_pattern(&mut self, input: Pair<Rule>) -> Pattern {
        let mut pattern = Pattern::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => pattern.value = self.parse_string(child),
                Rule::error_message => pattern.error_message = Some(self.parse_string(child)),
                Rule::error_app_tag => pattern.error_app_tag = Some(self.parse_string(child)),
                Rule::description => pattern.description = Some(self.parse_string(child)),
                Rule::reference => pattern.reference = Some(self.parse_string(child)),
                Rule::modifier => {
                    pattern.modifier = Some(
                        child
                            .into_inner()
                            .next()
                            .expect("modifier to always only have one child which, which is the invert-match string")
                            .as_str()
                            .to_string(),
                    )
                }
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        pattern
    }

    fn parse_length(&mut self, input: Pair<Rule>) -> Length {
        let mut length = Length::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => length.value = self.parse_string(child),
                Rule::error_message => length.error_message = Some(self.parse_string(child)),
                Rule::error_app_tag => length.error_app_tag = Some(self.parse_string(child)),
                Rule::description => length.description = Some(self.parse_string(child)),
                Rule::reference => length.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        length
    }

    fn parse_must(&mut self, input: Pair<Rule>) -> Must {
        let mut must = Must::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => must.condition = self.parse_string(child),
                Rule::error_message => must.error_message = Some(self.parse_string(child)),
                Rule::error_app_tag => must.error_app_tag = Some(self.parse_string(child)),
                Rule::description => must.description = Some(self.parse_string(child)),
                Rule::reference => must.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        must
    }

    fn parse_range(&mut self, input: Pair<Rule>) -> Range {
        let mut range = Range::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => range.value = self.parse_string(child),
                Rule::error_message => range.error_message = Some(self.parse_string(child)),
                Rule::error_app_tag => range.error_app_tag = Some(self.parse_string(child)),
                Rule::description => range.description = Some(self.parse_string(child)),
                Rule::reference => range.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        range
    }

    fn parse_status(&mut self, input: Pair<Rule>) -> Status {
        let status = input
            .into_inner()
            .next()
            .expect("status to always have a status_value as the only child");

        match status.as_str() {
            "current" => Status::Current,
            "obsolete" => Status::Obsolete,
            "deprecated" => Status::Deprecated,
            _ => unreachable!("Unexpected status: {:?}", status),
        }
    }

    fn parse_ordered_by(&mut self, input: Pair<Rule>) -> OrderedBy {
        let ordered_by = input
            .into_inner()
            .next()
            .expect("ordered-by to always have a ordered_by_value as the only child");

        match ordered_by.as_str() {
            "user" => OrderedBy::User,
            "system" => OrderedBy::System,
            _ => unreachable!("Unexpected ordered-by: {:?}", ordered_by),
        }
    }

    fn parse_max_elements(&mut self, input: Pair<Rule>) -> MaxElements {
        let max_elements = input
            .into_inner()
            .next()
            .expect("max-elements to always have a max_elements_value as the only child");

        match max_elements.as_rule() {
            Rule::integer => MaxElements::Value(self.parse_integer(max_elements)),
            Rule::string => MaxElements::Unbounded,
            _ => unreachable!("Unexpected rule: {:?}", max_elements),
        }
    }

    fn parse_revision(&mut self, input: Pair<Rule>) -> Revision {
        let mut revision = Revision::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => revision.date = self.parse_string(child),
                Rule::description => revision.description = Some(self.parse_string(child)),
                Rule::reference => revision.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        revision
    }

    fn parse_import(&mut self, input: Pair<Rule>) {
        let mut import = Import::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => import.module = self.parse_string(child),
                Rule::prefix => import.prefix = self.parse_string(child),
                Rule::revision_date => import.revision_date = Some(self.parse_string(child)),
                Rule::description => import.description = Some(self.parse_string(child)),
                Rule::reference => import.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        self.imports.push(import);
    }

    fn parse_include(&mut self, input: Pair<Rule>) {
        let mut include = Include::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => include.module = self.parse_string(child),
                Rule::revision_date => include.revision_date = Some(self.parse_string(child)),
                Rule::description => include.description = Some(self.parse_string(child)),
                Rule::reference => include.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        self.includes.push(include);
    }

    fn parse_boolean(&mut self, input: Pair<Rule>) -> bool {
        let value = input
            .into_inner()
            .next()
            .expect("boolean to always have the bool value as the only child");

        match value.as_str() {
            "true" => true,
            "false" => false,
            _ => unreachable!("Unexpected value: {:?}", value.as_str()),
        }
    }

    fn parse_integer(&mut self, input: Pair<Rule>) -> i64 {
        input
            .into_inner()
            .next()
            .expect("integer to always have the integers value as the only child")
            .as_str()
            .parse()
            .expect("integer value to always be a valid integer")
    }

    fn parse_string(&mut self, input: Pair<Rule>) -> String {
        let value = input
            .into_inner()
            .next()
            .expect("string to always have the string value as the only child");

        match value.as_rule() {
            Rule::string => self.parse_string(value),
            Rule::unquoted_string => value.as_str().to_string(),
            Rule::double_quoted_string => {
                let s = value.as_str();
                s[1..s.len() - 1].to_string()
            }
            Rule::single_quoted_string => {
                let s = value.as_str();
                s[1..s.len() - 1].to_string()
            }
            _ => unreachable!("Unexpected rule: {:?}", value.as_rule()),
        }
    }

    fn with_path_scope<F, T>(&mut self, name: String, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        let original_path_len = self.current_path.len();
        self.current_path.push_str(format!("{}/", &name).as_ref());

        let result = f(self);

        self.current_path.truncate(original_path_len);
        result
    }
}
