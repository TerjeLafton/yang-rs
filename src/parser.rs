use std::{collections::HashMap, fs, path::Path};

use pest::{error::Error, iterators::Pair, Parser};

use crate::{ast::*, resolver::ReferenceResolver, Rule, YangModule};

pub struct YangParser {
    current_path: String,
    groupings: HashMap<String, Grouping>,
}

impl YangParser {
    pub fn new() -> Self {
        Self {
            current_path: String::from("/"),
            groupings: HashMap::new(),
        }
    }

    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<YangFile, Error<Rule>> {
        let mut parser = Self::new();
        let content = fs::read_to_string(path).expect("file to be available");
        let mut parser_result = parser.parse(&content)?;

        let resolver = ReferenceResolver::new(parser.groupings);
        resolver.resolve_references(&mut parser_result);

        Ok(parser_result)
    }

    fn parse(&mut self, input: &str) -> Result<YangFile, Error<Rule>> {
        let module = YangModule::parse(Rule::file, input)?
            .next()
            .expect("a yang file to always include a module");

        match module.as_rule() {
            Rule::module => Ok(YangFile::Module(self.parse_module(module))),
            _ => unreachable!("Unexpected rule: {:?}", module.as_rule()),
        }
    }

    fn parse_module(&mut self, input: Pair<Rule>) -> Module {
        let pair = input;
        let mut module = Module::default();

        for child in pair.into_inner() {
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
                Rule::import => module.imports.push(self.parse_import(child)),
                Rule::include => module.includes.push(self.parse_include(child)),
                Rule::body => module.body.push(self.parse_body(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        module
    }

    fn parse_body(&mut self, input: Pair<Rule>) -> SchemaNode {
        let node = input.into_inner().next().expect("to always have inner nodes");

        match node.as_rule() {
            Rule::extension => SchemaNode::Extension(self.parse_extension(node)),
            Rule::feature => SchemaNode::Feature(self.parse_feature(node)),
            Rule::identity => SchemaNode::Identity(self.parse_identity(node)),
            Rule::type_def => SchemaNode::TypeDef(self.parse_type_def(node)),
            Rule::data_def => SchemaNode::DataDef(self.parse_data_def(node)),
            Rule::augment => SchemaNode::Augment(self.parse_augment(node)),
            Rule::rpc => SchemaNode::Rpc(self.parse_rpc(node)),
            Rule::notification => SchemaNode::Notification(self.parse_notification(node)),
            Rule::deviation => SchemaNode::Deviation(self.parse_deviation(node)),
            Rule::grouping => {
                self.parse_grouping(node);
                SchemaNode::Grouping
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

    fn parse_deviation(&mut self, input: Pair<Rule>) -> Deviation {
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

        deviation
    }

    fn parse_deviate_add(&mut self, input: Pair<Rule>) -> DeviateAdd {
        let mut deviate = DeviateAdd::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => deviate.target = self.parse_string(child),
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
                Rule::string => deviate.target = self.parse_string(child),
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
                Rule::string => deviate.target = self.parse_string(child),
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
                    Rule::type_def => container.type_defs.push(this.parse_type_def(child)),
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
                Rule::string => uses.grouping = self.parse_string(child),
                Rule::when => uses.when = Some(self.parse_when(child)),
                Rule::if_feature => uses.if_features.push(self.parse_string(child)),
                Rule::status => uses.status = Some(self.parse_status(child)),
                Rule::description => uses.description = Some(self.parse_string(child)),
                Rule::reference => uses.reference = Some(self.parse_string(child)),
                Rule::refine => uses.refines.push(self.parse_refine(child)),
                Rule::augment => uses.augments.push(self.parse_augment(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        uses
    }

    fn parse_augment(&mut self, input: Pair<Rule>) -> Augment {
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

        augment
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
                    Rule::type_def => output.type_defs.push(this.parse_type_def(child)),
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
                    Rule::type_def => new_input.type_defs.push(this.parse_type_def(child)),
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
                    Rule::type_def => rpc.type_defs.push(this.parse_type_def(child)),
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
                    Rule::type_def => action.type_defs.push(this.parse_type_def(child)),
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
                    Rule::type_def => notification.type_defs.push(this.parse_type_def(child)),
                    Rule::grouping => this.parse_grouping(child),
                    _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
                }
            }
        });

        notification
    }

    fn parse_feature(&mut self, input: Pair<Rule>) -> Feature {
        let mut feature = Feature::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => feature.name = self.parse_string(child),
                Rule::if_feature => feature.if_features.push(self.parse_string(child)),
                Rule::status => feature.status = Some(self.parse_status(child)),
                Rule::description => feature.description = Some(self.parse_string(child)),
                Rule::reference => feature.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        feature
    }

    fn parse_extension(&mut self, input: Pair<Rule>) -> Extension {
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

        extension
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
                    Rule::type_def => grouping.type_defs.push(this.parse_type_def(child)),
                    Rule::grouping => this.parse_grouping(child),
                    Rule::data_def => grouping.data_defs.push(this.parse_data_def(child)),
                    Rule::action => grouping.actions.push(this.parse_action(child)),
                    Rule::notification => grouping.notifications.push(this.parse_notification(child)),
                    _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
                }
            }
        });

        let path = format!("{}{}", self.current_path, grouping.name.clone());
        self.groupings.insert(path, grouping);
    }

    fn parse_type_def(&mut self, input: Pair<Rule>) -> TypeDef {
        let mut type_def = TypeDef::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => type_def.name = self.parse_string(child),
                Rule::type_info => type_def.type_info = self.parse_type_info(child),
                Rule::units => type_def.units = Some(self.parse_string(child)),
                Rule::default => type_def.default = Some(self.parse_string(child)),
                Rule::status => type_def.status = Some(self.parse_status(child)),
                Rule::description => type_def.description = Some(self.parse_string(child)),
                Rule::reference => type_def.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        type_def
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
                    Rule::type_def => list.type_defs.push(this.parse_type_def(child)),
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

    fn parse_identity(&mut self, input: Pair<Rule>) -> Identity {
        let mut identity = Identity::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => identity.name = self.parse_string(child),
                Rule::if_feature => identity.if_features.push(self.parse_string(child)),
                Rule::base => identity.bases.push(self.parse_string(child)),
                Rule::status => identity.status = Some(self.parse_status(child)),
                Rule::description => identity.description = Some(self.parse_string(child)),
                Rule::reference => identity.reference = Some(self.parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        identity
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

    fn parse_import(&mut self, input: Pair<Rule>) -> Import {
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

        import
    }

    fn parse_include(&mut self, input: Pair<Rule>) -> Include {
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

        include
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
