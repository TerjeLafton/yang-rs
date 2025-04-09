use std::{collections::HashMap, fs, path::Path};

use pest::{error::Error, iterators::Pair, Parser};

use crate::{ast::*, Rule, YangModule};

pub struct YangParser {
    pub groupings: HashMap<String, Grouping>,
    pub type_defs: HashMap<String, TypeDef>,
}

impl YangParser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<YangFile, Error<Rule>> {
        let content = fs::read_to_string(path).expect("file to be available");
        Self::parse(&content)
    }

    fn parse(input: &str) -> Result<YangFile, Error<Rule>> {
        let module = YangModule::parse(Rule::file, input)?
            .next()
            .expect("a yang file to always include a module");

        match module.as_rule() {
            Rule::module => Ok(YangFile::Module(Self::parse_module(module))),
            _ => unreachable!("Unexpected rule: {:?}", module.as_rule()),
        }
    }

    fn parse_module(input: Pair<Rule>) -> Module {
        let pair = input;
        let mut module = Module::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => module.name = Self::parse_string(child),
                Rule::prefix => module.prefix = Self::parse_string(child),
                Rule::namespace => module.namespace = Self::parse_string(child),
                Rule::yang_version => module.yang_version = Some(Self::parse_string(child)),
                Rule::organization => module.meta.organization = Some(Self::parse_string(child)),
                Rule::contact => module.meta.contact = Some(Self::parse_string(child)),
                Rule::description => module.meta.description = Some(Self::parse_string(child)),
                Rule::reference => module.meta.reference = Some(Self::parse_string(child)),
                Rule::revision => module.revisions.push(Self::parse_revision(child)),
                Rule::import => module.imports.push(Self::parse_import(child)),
                Rule::include => module.includes.push(Self::parse_include(child)),
                Rule::body => module.body.push(Self::parse_body(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        module
    }

    fn parse_body(input: Pair<Rule>) -> SchemaNode {
        let node = input.into_inner().next().expect("to always have inner nodes");

        match node.as_rule() {
            Rule::extension => SchemaNode::Extension(Self::parse_extension(node)),
            Rule::feature => SchemaNode::Feature(Self::parse_feature(node)),
            Rule::identity => SchemaNode::Identity(Self::parse_identity(node)),
            Rule::type_def => SchemaNode::TypeDef(Self::parse_type_def(node)),
            Rule::grouping => SchemaNode::Grouping(Self::parse_grouping(node)),
            Rule::data_def => SchemaNode::DataDef(Self::parse_data_def(node)),
            Rule::augment => SchemaNode::Augment(Self::parse_augment(node)),
            Rule::rpc => SchemaNode::Rpc(Self::parse_rpc(node)),
            Rule::notification => SchemaNode::Notification(Self::parse_notification(node)),
            Rule::deviation => SchemaNode::Deviation(Self::parse_deviation(node)),
            _ => unreachable!("Unexpected rule: {:?}", node.as_rule()),
        }
    }

    fn parse_anydata(input: Pair<Rule>) -> Anydata {
        let mut anydata = Anydata::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => anydata.name = Self::parse_string(child),
                Rule::when => anydata.when = Some(Self::parse_when(child)),
                Rule::if_feature => anydata.if_features.push(Self::parse_string(child)),
                Rule::must => anydata.must.push(Self::parse_must(child)),
                Rule::config => anydata.config = Some(Self::parse_boolean(child)),
                Rule::mandatory => anydata.mandatory = Some(Self::parse_boolean(child)),
                Rule::status => anydata.status = Some(Self::parse_status(child)),
                Rule::description => anydata.description = Some(Self::parse_string(child)),
                Rule::reference => anydata.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        anydata
    }

    fn parse_anyxml(input: Pair<Rule>) -> Anyxml {
        let mut anyxml = Anyxml::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => anyxml.name = Self::parse_string(child),
                Rule::when => anyxml.when = Some(Self::parse_when(child)),
                Rule::if_feature => anyxml.if_features.push(Self::parse_string(child)),
                Rule::must => anyxml.must.push(Self::parse_must(child)),
                Rule::config => anyxml.config = Some(Self::parse_boolean(child)),
                Rule::mandatory => anyxml.mandatory = Some(Self::parse_boolean(child)),
                Rule::status => anyxml.status = Some(Self::parse_status(child)),
                Rule::description => anyxml.description = Some(Self::parse_string(child)),
                Rule::reference => anyxml.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        anyxml
    }

    fn parse_deviation(input: Pair<Rule>) -> Deviation {
        let mut deviation = Deviation::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => deviation.target = Self::parse_string(child),
                Rule::description => deviation.description = Some(Self::parse_string(child)),
                Rule::reference => deviation.reference = Some(Self::parse_string(child)),
                Rule::deviation_not_supported => deviation.not_supported = true,
                Rule::deviate_add => deviation.add.push(Self::parse_deviate_add(child)),
                Rule::deviate_delete => deviation.delete.push(Self::parse_deviate_delete(child)),
                Rule::deviate_replace => deviation.replace.push(Self::parse_deviate_replace(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        deviation
    }

    fn parse_deviate_add(input: Pair<Rule>) -> DeviateAdd {
        let mut deviate = DeviateAdd::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => deviate.target = Self::parse_string(child),
                Rule::units => deviate.units = Some(Self::parse_string(child)),
                Rule::must => deviate.must.push(Self::parse_must(child)),
                Rule::unique => deviate.unique.push(Self::parse_string(child)),
                Rule::default => deviate.default.push(Self::parse_string(child)),
                Rule::config => deviate.config = Some(Self::parse_boolean(child)),
                Rule::mandatory => deviate.mandatory = Some(Self::parse_boolean(child)),
                Rule::min_elements => deviate.min_elements = Some(Self::parse_integer(child)),
                Rule::max_elements => deviate.max_elements = Some(Self::parse_max_elements(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        deviate
    }

    fn parse_deviate_delete(input: Pair<Rule>) -> DeviateDelete {
        let mut deviate = DeviateDelete::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => deviate.target = Self::parse_string(child),
                Rule::units => deviate.units = Some(Self::parse_string(child)),
                Rule::default => deviate.default.push(Self::parse_string(child)),
                Rule::must => deviate.must.push(Self::parse_must(child)),
                Rule::unique => deviate.unique.push(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        deviate
    }

    fn parse_deviate_replace(input: Pair<Rule>) -> DeviateReplace {
        let mut deviate = DeviateReplace::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => deviate.target = Self::parse_string(child),
                Rule::type_info => deviate.type_info = Some(Self::parse_type_info(child)),
                Rule::units => deviate.units = Some(Self::parse_string(child)),
                Rule::default => deviate.default.push(Self::parse_string(child)),
                Rule::config => deviate.config = Some(Self::parse_boolean(child)),
                Rule::mandatory => deviate.mandatory = Some(Self::parse_boolean(child)),
                Rule::min_elements => deviate.min_elements = Some(Self::parse_integer(child)),
                Rule::max_elements => deviate.max_elements = Some(Self::parse_max_elements(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        deviate
    }

    fn parse_data_def(input: Pair<Rule>) -> DataDef {
        let node = input.into_inner().next().expect("to always have inner nodes");

        match node.as_rule() {
            Rule::container => DataDef::Container(Self::parse_container(node)),
            Rule::leaf => DataDef::Leaf(Self::parse_leaf(node)),
            Rule::leaf_list => DataDef::LeafList(Self::parse_leaf_list(node)),
            Rule::list => DataDef::List(Self::parse_list(node)),
            Rule::choice => DataDef::Choice(Self::parse_choice(node)),
            Rule::anydata => DataDef::AnyData(Self::parse_anydata(node)),
            Rule::anyxml => DataDef::Anyxml(Self::parse_anyxml(node)),
            Rule::uses => DataDef::Uses(Self::parse_uses(node)),
            _ => unreachable!("Unexpected rule: {:?}", node.as_rule()),
        }
    }

    fn parse_container(input: Pair<Rule>) -> Container {
        let mut container = Container::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => container.name = Self::parse_string(child),
                Rule::when => container.when = Some(Self::parse_when(child)),
                Rule::if_feature => container.if_features.push(Self::parse_string(child)),
                Rule::must => container.must.push(Self::parse_must(child)),
                Rule::presence => container.presence = Some(Self::parse_string(child)),
                Rule::config => container.config = Some(Self::parse_boolean(child)),
                Rule::status => container.status = Some(Self::parse_status(child)),
                Rule::description => container.description = Some(Self::parse_string(child)),
                Rule::reference => container.reference = Some(Self::parse_string(child)),
                Rule::type_def => container.type_defs.push(Self::parse_type_def(child)),
                Rule::grouping => container.groupings.push(Self::parse_grouping(child)),
                Rule::data_def => container.data_defs.push(Self::parse_data_def(child)),
                Rule::action => container.actions.push(Self::parse_action(child)),
                Rule::notification => container.notifications.push(Self::parse_notification(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        container
    }

    fn parse_choice(input: Pair<Rule>) -> Choice {
        let mut choice = Choice::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => choice.name = Self::parse_string(child),
                Rule::when => choice.when = Some(Self::parse_when(child)),
                Rule::if_feature => choice.if_features.push(Self::parse_string(child)),
                Rule::default => choice.default = Some(Self::parse_string(child)),
                Rule::config => choice.config = Some(Self::parse_boolean(child)),
                Rule::mandatory => choice.mandatory = Some(Self::parse_boolean(child)),
                Rule::status => choice.status = Some(Self::parse_status(child)),
                Rule::description => choice.description = Some(Self::parse_string(child)),
                Rule::reference => choice.reference = Some(Self::parse_string(child)),
                Rule::long_case => choice.cases.push(Case::LongCase(Self::parse_long_case(child))),
                Rule::short_case => choice.cases.push(Case::ShortCase(Self::parse_short_case(child))),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        choice
    }

    fn parse_short_case(input: Pair<Rule>) -> ShortCase {
        let node = input.into_inner().next().expect("to always have inner node");

        match node.as_rule() {
            Rule::choice => ShortCase::Choice(Self::parse_choice(node)),
            Rule::container => ShortCase::Container(Self::parse_container(node)),
            Rule::leaf => ShortCase::Leaf(Self::parse_leaf(node)),
            Rule::leaf_list => ShortCase::LeafList(Self::parse_leaf_list(node)),
            Rule::list => ShortCase::List(Self::parse_list(node)),
            Rule::anydata => ShortCase::Anydata(Self::parse_anydata(node)),
            Rule::anyxml => ShortCase::Anyxml(Self::parse_anyxml(node)),
            _ => unreachable!("Unexpected rule: {:?}", node.as_rule()),
        }
    }

    fn parse_long_case(input: Pair<Rule>) -> LongCase {
        let mut case = LongCase::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => case.name = Self::parse_string(child),
                Rule::when => case.when = Some(Self::parse_when(child)),
                Rule::if_feature => case.if_features.push(Self::parse_string(child)),
                Rule::status => case.status = Some(Self::parse_status(child)),
                Rule::description => case.description = Some(Self::parse_string(child)),
                Rule::reference => case.reference = Some(Self::parse_string(child)),
                Rule::data_def => case.data_defs.push(Self::parse_data_def(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        case
    }

    fn parse_uses(input: Pair<Rule>) -> Uses {
        let mut uses = Uses::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => uses.grouping = Self::parse_string(child),
                Rule::when => uses.when = Some(Self::parse_when(child)),
                Rule::if_feature => uses.if_features.push(Self::parse_string(child)),
                Rule::status => uses.status = Some(Self::parse_status(child)),
                Rule::description => uses.description = Some(Self::parse_string(child)),
                Rule::reference => uses.reference = Some(Self::parse_string(child)),
                Rule::refine => uses.refines.push(Self::parse_refine(child)),
                Rule::augment => uses.augments.push(Self::parse_augment(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        uses
    }

    fn parse_augment(input: Pair<Rule>) -> Augment {
        let mut augment = Augment::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => augment.target = Self::parse_string(child),
                Rule::when => augment.when = Some(Self::parse_when(child)),
                Rule::if_feature => augment.if_features.push(Self::parse_string(child)),
                Rule::status => augment.status = Some(Self::parse_status(child)),
                Rule::description => augment.description = Some(Self::parse_string(child)),
                Rule::reference => augment.reference = Some(Self::parse_string(child)),
                Rule::data_def => augment.data_defs.push(Self::parse_data_def(child)),
                Rule::long_case => augment.cases.push(Case::LongCase(Self::parse_long_case(child))),
                Rule::action => augment.actions.push(Self::parse_action(child)),
                Rule::notification => augment.notifications.push(Self::parse_notification(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        augment
    }

    fn parse_refine(input: Pair<Rule>) -> Refine {
        let mut refine = Refine::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => refine.target = Self::parse_string(child),
                Rule::if_feature => refine.if_features.push(Self::parse_string(child)),
                Rule::must => refine.must.push(Self::parse_must(child)),
                Rule::presence => refine.presence = Some(Self::parse_string(child)),
                Rule::default => refine.default.push(Self::parse_string(child)),
                Rule::config => refine.config = Some(Self::parse_boolean(child)),
                Rule::mandatory => refine.mandatory = Some(Self::parse_boolean(child)),
                Rule::min_elements => refine.min_elements = Some(Self::parse_integer(child)),
                Rule::max_elements => refine.max_elements = Some(Self::parse_max_elements(child)),
                Rule::description => refine.description = Some(Self::parse_string(child)),
                Rule::reference => refine.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        refine
    }

    fn parse_output(input: Pair<Rule>) -> Output {
        let mut output = Output::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::must => output.must.push(Self::parse_must(child)),
                Rule::type_def => output.type_defs.push(Self::parse_type_def(child)),
                Rule::grouping => output.groupings.push(Self::parse_grouping(child)),
                Rule::data_def => output.data_defs.push(Self::parse_data_def(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        output
    }

    fn parse_input(input: Pair<Rule>) -> Input {
        let mut new_input = Input::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::must => new_input.must.push(Self::parse_must(child)),
                Rule::type_def => new_input.type_defs.push(Self::parse_type_def(child)),
                Rule::grouping => new_input.groupings.push(Self::parse_grouping(child)),
                Rule::data_def => new_input.data_defs.push(Self::parse_data_def(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        new_input
    }

    fn parse_rpc(input: Pair<Rule>) -> Rpc {
        let mut rpc = Rpc::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => rpc.name = Self::parse_string(child),
                Rule::input => rpc.input = Some(Self::parse_input(child)),
                Rule::output => rpc.output = Some(Self::parse_output(child)),
                Rule::if_feature => rpc.if_features.push(Self::parse_string(child)),
                Rule::must => rpc.must.push(Self::parse_must(child)),
                Rule::status => rpc.status = Some(Self::parse_status(child)),
                Rule::description => rpc.description = Some(Self::parse_string(child)),
                Rule::reference => rpc.reference = Some(Self::parse_string(child)),
                Rule::type_def => rpc.type_defs.push(Self::parse_type_def(child)),
                Rule::grouping => rpc.groupings.push(Self::parse_grouping(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        rpc
    }

    fn parse_action(input: Pair<Rule>) -> Action {
        let mut action = Action::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => action.name = Self::parse_string(child),
                Rule::input => action.input = Some(Self::parse_input(child)),
                Rule::output => action.output = Some(Self::parse_output(child)),
                Rule::if_feature => action.if_features.push(Self::parse_string(child)),
                Rule::must => action.must.push(Self::parse_must(child)),
                Rule::status => action.status = Some(Self::parse_status(child)),
                Rule::description => action.description = Some(Self::parse_string(child)),
                Rule::reference => action.reference = Some(Self::parse_string(child)),
                Rule::type_def => action.type_defs.push(Self::parse_type_def(child)),
                Rule::grouping => action.groupings.push(Self::parse_grouping(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        action
    }

    fn parse_notification(input: Pair<Rule>) -> Notification {
        let mut notification = Notification::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => notification.name = Self::parse_string(child),
                Rule::data_def => notification.data_defs.push(Self::parse_data_def(child)),
                Rule::if_feature => notification.if_features.push(Self::parse_string(child)),
                Rule::must => notification.must.push(Self::parse_must(child)),
                Rule::status => notification.status = Some(Self::parse_status(child)),
                Rule::description => notification.description = Some(Self::parse_string(child)),
                Rule::reference => notification.reference = Some(Self::parse_string(child)),
                Rule::type_def => notification.type_defs.push(Self::parse_type_def(child)),
                Rule::grouping => notification.groupings.push(Self::parse_grouping(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        notification
    }

    fn parse_feature(input: Pair<Rule>) -> Feature {
        let mut feature = Feature::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => feature.name = Self::parse_string(child),
                Rule::if_feature => feature.if_features.push(Self::parse_string(child)),
                Rule::status => feature.status = Some(Self::parse_status(child)),
                Rule::description => feature.description = Some(Self::parse_string(child)),
                Rule::reference => feature.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        feature
    }

    fn parse_extension(input: Pair<Rule>) -> Extension {
        let mut extension = Extension::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => extension.name = Self::parse_string(child),
                Rule::argument => extension.argument = Some(Self::parse_argument(child)),
                Rule::status => extension.status = Some(Self::parse_status(child)),
                Rule::description => extension.description = Some(Self::parse_string(child)),
                Rule::reference => extension.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        extension
    }

    fn parse_when(input: Pair<Rule>) -> When {
        let mut when = When::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => when.condition = Self::parse_string(child),
                Rule::description => when.description = Some(Self::parse_string(child)),
                Rule::reference => when.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        when
    }

    fn parse_argument(input: Pair<Rule>) -> Argument {
        let mut argument = Argument::default();
        let mut input = input.into_inner();

        argument.name = Self::parse_string(input.next().expect("first child to always be the name"));
        if let Some(yin_element) = input.next() {
            argument.yin_element = Some(Self::parse_boolean(yin_element))
        }

        argument
    }

    fn parse_grouping(input: Pair<Rule>) -> Grouping {
        let mut grouping = Grouping::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => grouping.name = Self::parse_string(child),
                Rule::status => grouping.status = Some(Self::parse_status(child)),
                Rule::description => grouping.description = Some(Self::parse_string(child)),
                Rule::reference => grouping.reference = Some(Self::parse_string(child)),
                Rule::type_def => grouping.type_defs.push(Self::parse_type_def(child)),
                Rule::grouping => grouping.groupings.push(Self::parse_grouping(child)),
                Rule::data_def => grouping.data_defs.push(Self::parse_data_def(child)),
                Rule::action => grouping.actions.push(Self::parse_action(child)),
                Rule::notification => grouping.notifications.push(Self::parse_notification(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        grouping
    }

    fn parse_type_def(input: Pair<Rule>) -> TypeDef {
        let mut type_def = TypeDef::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => type_def.name = Self::parse_string(child),
                Rule::type_info => type_def.type_info = Self::parse_type_info(child),
                Rule::units => type_def.units = Some(Self::parse_string(child)),
                Rule::default => type_def.default = Some(Self::parse_string(child)),
                Rule::status => type_def.status = Some(Self::parse_status(child)),
                Rule::description => type_def.description = Some(Self::parse_string(child)),
                Rule::reference => type_def.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        type_def
    }
    fn parse_leaf_list(input: Pair<Rule>) -> LeafList {
        let mut leaf_list = LeafList::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => leaf_list.name = Self::parse_string(child),
                Rule::when => leaf_list.when = Some(Self::parse_when(child)),
                Rule::if_feature => leaf_list.if_features.push(Self::parse_string(child)),
                Rule::type_info => leaf_list.type_info = Self::parse_type_info(child),
                Rule::units => leaf_list.units = Some(Self::parse_string(child)),
                Rule::must => leaf_list.must.push(Self::parse_must(child)),
                Rule::default => leaf_list.default.push(Self::parse_string(child)),
                Rule::config => leaf_list.config = Some(Self::parse_boolean(child)),
                Rule::ordered_by => leaf_list.ordered_by = Some(Self::parse_ordered_by(child)),
                Rule::min_elements => leaf_list.min_elements = Some(Self::parse_integer(child)),
                Rule::max_elements => leaf_list.max_elements = Some(Self::parse_max_elements(child)),
                Rule::status => leaf_list.status = Some(Self::parse_status(child)),
                Rule::description => leaf_list.description = Some(Self::parse_string(child)),
                Rule::reference => leaf_list.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        leaf_list
    }

    fn parse_leaf(input: Pair<Rule>) -> Leaf {
        let mut leaf = Leaf::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => leaf.name = Self::parse_string(child),
                Rule::when => leaf.when = Some(Self::parse_when(child)),
                Rule::if_feature => leaf.if_features.push(Self::parse_string(child)),
                Rule::type_info => leaf.type_info = Self::parse_type_info(child),
                Rule::units => leaf.units = Some(Self::parse_string(child)),
                Rule::must => leaf.must.push(Self::parse_must(child)),
                Rule::default => leaf.default = Some(Self::parse_string(child)),
                Rule::config => leaf.config = Some(Self::parse_boolean(child)),
                Rule::mandatory => leaf.mandatory = Some(Self::parse_boolean(child)),
                Rule::status => leaf.status = Some(Self::parse_status(child)),
                Rule::description => leaf.description = Some(Self::parse_string(child)),
                Rule::reference => leaf.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        leaf
    }

    fn parse_list(input: Pair<Rule>) -> List {
        let mut list = List::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => list.name = Self::parse_string(child),
                Rule::when => list.when = Some(Self::parse_when(child)),
                Rule::if_feature => list.if_features.push(Self::parse_string(child)),
                Rule::must => list.must.push(Self::parse_must(child)),
                Rule::key => list.key = Some(Self::parse_string(child)),
                Rule::unique => list.unique.push(Self::parse_string(child)),
                Rule::config => list.config = Some(Self::parse_boolean(child)),
                Rule::min_elements => list.min_elements = Some(Self::parse_integer(child)),
                Rule::max_elements => list.max_elements = Some(Self::parse_max_elements(child)),
                Rule::ordered_by => list.ordered_by = Some(Self::parse_ordered_by(child)),
                Rule::status => list.status = Some(Self::parse_status(child)),
                Rule::description => list.description = Some(Self::parse_string(child)),
                Rule::reference => list.reference = Some(Self::parse_string(child)),
                Rule::type_def => list.type_defs.push(Self::parse_type_def(child)),
                Rule::grouping => list.groupings.push(Self::parse_grouping(child)),
                Rule::data_def => list.data_defs.push(Self::parse_data_def(child)),
                Rule::action => list.actions.push(Self::parse_action(child)),
                Rule::notification => list.notifications.push(Self::parse_notification(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        list
    }

    fn parse_type_info(input: Pair<Rule>) -> TypeInfo {
        let mut type_info = TypeInfo::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => type_info.name = Self::parse_string(child),
                Rule::numberical_restriction => type_info.type_body = Some(Self::parse_numerical(child)),
                Rule::decimal64_specification => type_info.type_body = Some(Self::parse_decimal(child)),
                Rule::string_restriction => type_info.type_body = Some(Self::parse_string_restriction(child)),
                Rule::enum_specification => type_info.type_body = Some(Self::parse_enum(child)),
                Rule::leafref_specification => type_info.type_body = Some(Self::parse_leafref(child)),
                Rule::identityref_specification => type_info.type_body = Some(Self::parse_identityref(child)),
                Rule::bits_specification => type_info.type_body = Some(Self::parse_bit_specification(child)),
                Rule::binary_specification => type_info.type_body = Some(Self::parse_binary_specification(child)),
                Rule::union_specification => type_info.type_body = Some(Self::parse_union_specification(child)),
                Rule::instance_identifier_specification => {
                    type_info.type_body = Some(Self::parse_instance_identifier(child))
                }
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        return type_info;
    }

    fn parse_union_specification(input: Pair<Rule>) -> TypeBody {
        let mut types = Vec::new();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::type_info => types.push(Self::parse_type_info(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        TypeBody::Union { types }
    }

    fn parse_binary_specification(input: Pair<Rule>) -> TypeBody {
        match input.into_inner().next() {
            Some(length) => TypeBody::Binary {
                length: Some(Self::parse_length(length)),
            },
            None => TypeBody::Binary { length: None },
        }
    }

    fn parse_bit_specification(input: Pair<Rule>) -> TypeBody {
        let mut bits = Vec::new();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::bit => bits.push(Self::parse_bit(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        TypeBody::Bits { bits }
    }

    fn parse_identity(input: Pair<Rule>) -> Identity {
        let mut identity = Identity::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => identity.name = Self::parse_string(child),
                Rule::if_feature => identity.if_features.push(Self::parse_string(child)),
                Rule::base => identity.bases.push(Self::parse_string(child)),
                Rule::status => identity.status = Some(Self::parse_status(child)),
                Rule::description => identity.description = Some(Self::parse_string(child)),
                Rule::reference => identity.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        identity
    }

    fn parse_identityref(input: Pair<Rule>) -> TypeBody {
        let mut bases = Vec::new();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::base => bases.push(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        TypeBody::Identityref { bases }
    }

    fn parse_instance_identifier(input: Pair<Rule>) -> TypeBody {
        TypeBody::InstanceIdentifier {
            require_instance: Self::parse_boolean(
                input
                    .into_inner()
                    .next()
                    .expect("instance identifier to always have a require instance node"),
            ),
        }
    }

    fn parse_leafref(input: Pair<Rule>) -> TypeBody {
        let mut leafref = input.into_inner();
        let path = Self::parse_string(leafref.next().expect("first child of leafref to be the path"));

        match leafref.next() {
            Some(require_instance) => TypeBody::Leafref {
                path,
                require_instance: Some(Self::parse_boolean(require_instance)),
            },
            None => TypeBody::Leafref {
                path,
                require_instance: None,
            },
        }
    }

    fn parse_string_restriction(input: Pair<Rule>) -> TypeBody {
        let mut length = None;
        let mut patterns = Vec::new();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::length => length = Some(Self::parse_length(child)),
                Rule::pattern => patterns.push(Self::parse_pattern(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        TypeBody::String { length, patterns }
    }

    fn parse_decimal(input: Pair<Rule>) -> TypeBody {
        let mut decimal_node = input.into_inner();
        let fractional_digits = Self::parse_string(
            decimal_node
                .next()
                .expect("decimal's first child always to be a fractional_digits node"),
        );

        match decimal_node.next() {
            Some(range) => TypeBody::Decimal64 {
                fraction_digits: fractional_digits,
                range: Some(Self::parse_range(range)),
            },
            None => TypeBody::Decimal64 {
                fraction_digits: fractional_digits,
                range: None,
            },
        }
    }

    fn parse_numerical(input: Pair<Rule>) -> TypeBody {
        TypeBody::Numerical {
            range: Self::parse_range(
                input
                    .into_inner()
                    .next()
                    .expect("numerical to always have range as the only child"),
            ),
        }
    }

    fn parse_enum(input: Pair<Rule>) -> TypeBody {
        let mut enums = Vec::new();
        for enum_child in input.into_inner() {
            let mut enum_value = EnumValue::default();
            for child in enum_child.into_inner() {
                match child.as_rule() {
                    Rule::string => enum_value.name = Self::parse_string(child),
                    Rule::if_feature => enum_value.if_features.push(Self::parse_string(child)),
                    Rule::value => enum_value.value = Some(Self::parse_integer(child)),
                    Rule::status => enum_value.status = Some(Self::parse_status(child)),
                    Rule::description => enum_value.description = Some(Self::parse_string(child)),
                    Rule::reference => enum_value.reference = Some(Self::parse_string(child)),
                    _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
                }
            }
            enums.push(enum_value);
        }

        TypeBody::Enum { enums }
    }

    fn parse_bit(input: Pair<Rule>) -> Bit {
        let mut bit = Bit::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => bit.name = Self::parse_string(child),
                Rule::if_feature => bit.if_features.push(Self::parse_string(child)),
                Rule::position => bit.position = Some(Self::parse_integer(child)),
                Rule::status => bit.status = Some(Self::parse_status(child)),
                Rule::description => bit.description = Some(Self::parse_string(child)),
                Rule::reference => bit.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        bit
    }

    fn parse_pattern(input: Pair<Rule>) -> Pattern {
        let mut pattern = Pattern::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => pattern.value = Self::parse_string(child),
                Rule::error_message => pattern.error_message = Some(Self::parse_string(child)),
                Rule::error_app_tag => pattern.error_app_tag = Some(Self::parse_string(child)),
                Rule::description => pattern.description = Some(Self::parse_string(child)),
                Rule::reference => pattern.reference = Some(Self::parse_string(child)),
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

    fn parse_length(input: Pair<Rule>) -> Length {
        let mut length = Length::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => length.value = Self::parse_string(child),
                Rule::error_message => length.error_message = Some(Self::parse_string(child)),
                Rule::error_app_tag => length.error_app_tag = Some(Self::parse_string(child)),
                Rule::description => length.description = Some(Self::parse_string(child)),
                Rule::reference => length.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        length
    }

    fn parse_must(input: Pair<Rule>) -> Must {
        let mut must = Must::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => must.condition = Self::parse_string(child),
                Rule::error_message => must.error_message = Some(Self::parse_string(child)),
                Rule::error_app_tag => must.error_app_tag = Some(Self::parse_string(child)),
                Rule::description => must.description = Some(Self::parse_string(child)),
                Rule::reference => must.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        must
    }

    fn parse_range(input: Pair<Rule>) -> Range {
        let mut range = Range::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => range.value = Self::parse_string(child),
                Rule::error_message => range.error_message = Some(Self::parse_string(child)),
                Rule::error_app_tag => range.error_app_tag = Some(Self::parse_string(child)),
                Rule::description => range.description = Some(Self::parse_string(child)),
                Rule::reference => range.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        range
    }

    fn parse_status(input: Pair<Rule>) -> Status {
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

    fn parse_ordered_by(input: Pair<Rule>) -> OrderedBy {
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

    fn parse_max_elements(input: Pair<Rule>) -> MaxElements {
        let max_elements = input
            .into_inner()
            .next()
            .expect("max-elements to always have a max_elements_value as the only child");

        match max_elements.as_rule() {
            Rule::integer => MaxElements::Value(Self::parse_integer(max_elements)),
            Rule::string => MaxElements::Unbounded,
            _ => unreachable!("Unexpected rule: {:?}", max_elements),
        }
    }

    fn parse_revision(input: Pair<Rule>) -> Revision {
        let mut revision = Revision::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => revision.date = Self::parse_string(child),
                Rule::description => revision.description = Some(Self::parse_string(child)),
                Rule::reference => revision.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        revision
    }

    fn parse_import(input: Pair<Rule>) -> Import {
        let mut import = Import::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => import.module = Self::parse_string(child),
                Rule::prefix => import.prefix = Self::parse_string(child),
                Rule::revision_date => import.revision_date = Some(Self::parse_string(child)),
                Rule::description => import.description = Some(Self::parse_string(child)),
                Rule::reference => import.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        import
    }

    fn parse_include(input: Pair<Rule>) -> Include {
        let mut include = Include::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => include.module = Self::parse_string(child),
                Rule::revision_date => include.revision_date = Some(Self::parse_string(child)),
                Rule::description => include.description = Some(Self::parse_string(child)),
                Rule::reference => include.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        include
    }

    fn parse_boolean(input: Pair<Rule>) -> bool {
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

    fn parse_integer(input: Pair<Rule>) -> i64 {
        input
            .into_inner()
            .next()
            .expect("integer to always have the integers value as the only child")
            .as_str()
            .parse()
            .expect("integer value to always be a valid integer")
    }

    fn parse_string(input: Pair<Rule>) -> String {
        let value = input
            .into_inner()
            .next()
            .expect("string to always have the string value as the only child");

        match value.as_rule() {
            Rule::string => Self::parse_string(value),
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
}
