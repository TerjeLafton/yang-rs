use crate::{Rule, parse::Parse};
use pest::iterators::Pair;

use crate::parse::helpers::*;

/// Represents a complete YANG module or submodule
#[derive(Debug, Clone)]
pub enum YangFile {
    Module(Module),
    Submodule(Submodule),
}

impl Parse for YangFile {
    fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::module => YangFile::Module(Module::parse(pair)),
            Rule::submodule => YangFile::Submodule(Submodule::parse(pair)),
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }
}

/// Represents a YANG module
#[derive(Debug, Clone, Default)]
pub struct Module {
    pub name: String,
    pub yang_version: Option<String>,
    pub namespace: String,
    pub prefix: String,
    pub imports: Vec<Import>,
    pub includes: Vec<Include>,
    pub meta: MetaInfo,
    pub revisions: Vec<Revision>,
    pub body: Vec<SchemaNode>,
}

impl Parse for Module {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut module = Module::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => module.name = parse_string(child),
                Rule::prefix => module.prefix = parse_string(child),
                Rule::namespace => module.namespace = parse_string(child),
                Rule::yang_version => module.yang_version = Some(parse_string(child)),
                Rule::organization => module.meta.organization = Some(parse_string(child)),
                Rule::contact => module.meta.contact = Some(parse_string(child)),
                Rule::description => module.meta.description = Some(parse_string(child)),
                Rule::reference => module.meta.reference = Some(parse_string(child)),
                Rule::revision => module.revisions.push(Revision::parse(child)),
                Rule::import => module.imports.push(Import::parse(child)),
                Rule::include => module.includes.push(Include::parse(child)),
                Rule::body => module.body.push(SchemaNode::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        module
    }
}

/// Represents a YANG submodule
#[derive(Debug, Clone, Default)]
pub struct Submodule {
    pub name: String,
    pub yang_version: Option<String>,
    pub belongs_to: BelongsTo,
    pub imports: Vec<Import>,
    pub includes: Vec<Include>,
    pub meta: MetaInfo,
    pub revisions: Vec<Revision>,
    pub body: Vec<SchemaNode>,
}

impl Parse for Submodule {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut submodule = Submodule::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => submodule.name = parse_string(child),
                Rule::belongs_to => submodule.belongs_to = BelongsTo::parse(child),
                Rule::yang_version => submodule.yang_version = Some(parse_string(child)),
                Rule::organization => submodule.meta.organization = Some(parse_string(child)),
                Rule::contact => submodule.meta.contact = Some(parse_string(child)),
                Rule::description => submodule.meta.description = Some(parse_string(child)),
                Rule::reference => submodule.meta.reference = Some(parse_string(child)),
                Rule::revision => submodule.revisions.push(Revision::parse(child)),
                Rule::import => submodule.imports.push(Import::parse(child)),
                Rule::include => submodule.includes.push(Include::parse(child)),
                Rule::body => submodule.body.push(SchemaNode::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        submodule
    }
}

#[derive(Debug, Clone, Default)]
pub struct BelongsTo {
    pub module: String,
    pub prefix: String,
}

impl Parse for BelongsTo {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut belongs_to = BelongsTo::default();
        let mut input = pair.into_inner();
        
        belongs_to.module = parse_string(input.next().expect("belongs-to to have a module name"));
        belongs_to.prefix = parse_string(input.next().expect("belongs-to to have a prefix"));
        
        belongs_to
    }
}

/// Import statement
#[derive(Debug, Clone, Default)]
pub struct Import {
    pub module: String,
    pub prefix: String,
    pub revision_date: Option<String>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for Import {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut import = Import::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => import.module = parse_string(child),
                Rule::prefix => import.prefix = parse_string(child),
                Rule::revision_date => import.revision_date = Some(parse_string(child)),
                Rule::description => import.description = Some(parse_string(child)),
                Rule::reference => import.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        import
    }
}

/// Include statement
#[derive(Debug, Clone, Default)]
pub struct Include {
    pub module: String,
    pub revision_date: Option<String>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for Include {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut include = Include::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => include.module = parse_string(child),
                Rule::revision_date => include.revision_date = Some(parse_string(child)),
                Rule::description => include.description = Some(parse_string(child)),
                Rule::reference => include.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        include
    }
}

/// Meta information for modules
#[derive(Debug, Clone, Default)]
pub struct MetaInfo {
    pub organization: Option<String>,
    pub contact: Option<String>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for MetaInfo {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut meta = MetaInfo::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::organization => meta.organization = Some(parse_string(child)),
                Rule::contact => meta.contact = Some(parse_string(child)),
                Rule::description => meta.description = Some(parse_string(child)),
                Rule::reference => meta.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        meta
    }
}

/// Revision history
#[derive(Debug, Clone, Default)]
pub struct Revision {
    pub date: String,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for Revision {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut revision = Revision::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => revision.date = parse_string(child),
                Rule::description => revision.description = Some(parse_string(child)),
                Rule::reference => revision.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        revision
    }
}

/// All possible schema nodes that can appear in a YANG module body
#[derive(Debug, Clone)]
pub enum SchemaNode {
    TypeDef(TypeDef),
    Grouping(Grouping),
    Extension(Extension),
    Feature(Feature),
    Identity(Identity),
    Augment(Augment),
    Rpc(Rpc),
    Notification(Notification),
    Deviation(Deviation),
    DataDef(DataDef),
}

impl Parse for SchemaNode {
    fn parse(pair: Pair<Rule>) -> Self {
        let node = pair.into_inner().next().expect("to always have inner nodes");

        match node.as_rule() {
            Rule::extension => SchemaNode::Extension(Extension::parse(node)),
            Rule::feature => SchemaNode::Feature(Feature::parse(node)),
            Rule::identity => SchemaNode::Identity(Identity::parse(node)),
            Rule::type_def => SchemaNode::TypeDef(TypeDef::parse(node)),
            Rule::grouping => SchemaNode::Grouping(Grouping::parse(node)),
            Rule::data_def => SchemaNode::DataDef(DataDef::parse(node)),
            Rule::augment => SchemaNode::Augment(Augment::parse(node)),
            Rule::rpc => SchemaNode::Rpc(Rpc::parse(node)),
            Rule::notification => SchemaNode::Notification(Notification::parse(node)),
            Rule::deviation => SchemaNode::Deviation(Deviation::parse(node)),
            _ => unreachable!("Unexpected rule: {:?}", node.as_rule()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DataDef {
    Container(Container),
    Leaf(Leaf),
    LeafList(LeafList),
    List(List),
    Choice(Choice),
    AnyData(Anydata),
    Anyxml(Anyxml),
    Uses(Uses),
}

impl Parse for DataDef {
    fn parse(pair: Pair<Rule>) -> Self {
        let node = pair.into_inner().next().expect("to always have inner nodes");

        match node.as_rule() {
            Rule::container => DataDef::Container(Container::parse(node)),
            Rule::leaf => DataDef::Leaf(Leaf::parse(node)),
            Rule::leaf_list => DataDef::LeafList(LeafList::parse(node)),
            Rule::list => DataDef::List(List::parse(node)),
            Rule::choice => DataDef::Choice(Choice::parse(node)),
            Rule::anydata => DataDef::AnyData(Anydata::parse(node)),
            Rule::anyxml => DataDef::Anyxml(Anyxml::parse(node)),
            Rule::uses => DataDef::Uses(Uses::parse(node)),
            _ => unreachable!("Unexpected rule: {:?}", node.as_rule()),
        }
    }
}

/// Container statement
#[derive(Debug, Clone, Default)]
pub struct Container {
    pub name: String,
    pub when: Option<When>,
    pub if_features: Vec<String>,
    pub must: Vec<Must>,
    pub presence: Option<String>,
    pub config: Option<bool>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub type_defs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub data_defs: Vec<DataDef>,
    pub actions: Vec<Action>,
    pub notifications: Vec<Notification>,
}

impl Parse for Container {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut container = Container::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => container.name = parse_string(child),
                Rule::when => container.when = Some(When::parse(child)),
                Rule::if_feature => container.if_features.push(parse_string(child)),
                Rule::must => container.must.push(Must::parse(child)),
                Rule::presence => container.presence = Some(parse_string(child)),
                Rule::config => container.config = Some(parse_boolean(child)),
                Rule::status => container.status = Some(Status::parse(child)),
                Rule::description => container.description = Some(parse_string(child)),
                Rule::reference => container.reference = Some(parse_string(child)),
                Rule::type_def => container.type_defs.push(TypeDef::parse(child)),
                Rule::grouping => container.groupings.push(Grouping::parse(child)),
                Rule::data_def => container.data_defs.push(DataDef::parse(child)),
                Rule::action => container.actions.push(Action::parse(child)),
                Rule::notification => container.notifications.push(Notification::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        container
    }
}

/// Leaf statement
#[derive(Debug, Clone, Default)]
pub struct Leaf {
    pub name: String,
    pub when: Option<When>,
    pub if_features: Vec<String>,
    pub type_info: TypeInfo,
    pub units: Option<String>,
    pub must: Vec<Must>,
    pub default: Option<String>,
    pub config: Option<bool>,
    pub mandatory: Option<bool>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for Leaf {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut leaf = Leaf::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => leaf.name = parse_string(child),
                Rule::when => leaf.when = Some(When::parse(child)),
                Rule::if_feature => leaf.if_features.push(parse_string(child)),
                Rule::type_info => leaf.type_info = TypeInfo::parse(child),
                Rule::units => leaf.units = Some(parse_string(child)),
                Rule::must => leaf.must.push(Must::parse(child)),
                Rule::default => leaf.default = Some(parse_string(child)),
                Rule::config => leaf.config = Some(parse_boolean(child)),
                Rule::mandatory => leaf.mandatory = Some(parse_boolean(child)),
                Rule::status => leaf.status = Some(Status::parse(child)),
                Rule::description => leaf.description = Some(parse_string(child)),
                Rule::reference => leaf.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        leaf
    }
}

/// Leaf-list statement
#[derive(Debug, Clone, Default)]
pub struct LeafList {
    pub name: String,
    pub when: Option<When>,
    pub if_features: Vec<String>,
    pub type_info: TypeInfo,
    pub units: Option<String>,
    pub must: Vec<Must>,
    pub default: Vec<String>,
    pub config: Option<bool>,
    pub min_elements: Option<i64>,
    pub max_elements: Option<MaxElements>,
    pub ordered_by: Option<OrderedBy>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for LeafList {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut leaf_list = LeafList::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => leaf_list.name = parse_string(child),
                Rule::when => leaf_list.when = Some(When::parse(child)),
                Rule::if_feature => leaf_list.if_features.push(parse_string(child)),
                Rule::type_info => leaf_list.type_info = TypeInfo::parse(child),
                Rule::units => leaf_list.units = Some(parse_string(child)),
                Rule::must => leaf_list.must.push(Must::parse(child)),
                Rule::default => leaf_list.default.push(parse_string(child)),
                Rule::config => leaf_list.config = Some(parse_boolean(child)),
                Rule::ordered_by => leaf_list.ordered_by = Some(OrderedBy::parse(child)),
                Rule::min_elements => leaf_list.min_elements = Some(parse_integer(child)),
                Rule::max_elements => leaf_list.max_elements = Some(MaxElements::parse(child)),
                Rule::status => leaf_list.status = Some(Status::parse(child)),
                Rule::description => leaf_list.description = Some(parse_string(child)),
                Rule::reference => leaf_list.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        leaf_list
    }
}

/// List statement
#[derive(Debug, Clone, Default)]
pub struct List {
    pub name: String,
    pub when: Option<When>,
    pub if_features: Vec<String>,
    pub must: Vec<Must>,
    pub key: Option<String>,
    pub unique: Vec<String>,
    pub config: Option<bool>,
    pub min_elements: Option<i64>,
    pub max_elements: Option<MaxElements>,
    pub ordered_by: Option<OrderedBy>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub type_defs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub data_defs: Vec<DataDef>,
    pub actions: Vec<Action>,
    pub notifications: Vec<Notification>,
}

impl Parse for List {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut list = List::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => list.name = parse_string(child),
                Rule::when => list.when = Some(When::parse(child)),
                Rule::if_feature => list.if_features.push(parse_string(child)),
                Rule::must => list.must.push(Must::parse(child)),
                Rule::key => list.key = Some(parse_string(child)),
                Rule::unique => list.unique.push(parse_string(child)),
                Rule::config => list.config = Some(parse_boolean(child)),
                Rule::min_elements => list.min_elements = Some(parse_integer(child)),
                Rule::max_elements => list.max_elements = Some(MaxElements::parse(child)),
                Rule::ordered_by => list.ordered_by = Some(OrderedBy::parse(child)),
                Rule::status => list.status = Some(Status::parse(child)),
                Rule::description => list.description = Some(parse_string(child)),
                Rule::reference => list.reference = Some(parse_string(child)),
                Rule::type_def => list.type_defs.push(TypeDef::parse(child)),
                Rule::grouping => list.groupings.push(Grouping::parse(child)),
                Rule::data_def => list.data_defs.push(DataDef::parse(child)),
                Rule::action => list.actions.push(Action::parse(child)),
                Rule::notification => list.notifications.push(Notification::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        list
    }
}

/// Choice statement
#[derive(Debug, Clone, Default)]
pub struct Choice {
    pub name: String,
    pub when: Option<When>,
    pub if_features: Vec<String>,
    pub default: Option<String>,
    pub config: Option<bool>,
    pub mandatory: Option<bool>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub cases: Vec<Case>,
}

impl Parse for Choice {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut choice = Choice::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => choice.name = parse_string(child),
                Rule::when => choice.when = Some(When::parse(child)),
                Rule::if_feature => choice.if_features.push(parse_string(child)),
                Rule::default => choice.default = Some(parse_string(child)),
                Rule::config => choice.config = Some(parse_boolean(child)),
                Rule::mandatory => choice.mandatory = Some(parse_boolean(child)),
                Rule::status => choice.status = Some(Status::parse(child)),
                Rule::description => choice.description = Some(parse_string(child)),
                Rule::reference => choice.reference = Some(parse_string(child)),
                Rule::long_case => choice.cases.push(Case::LongCase(LongCase::parse(child))),
                Rule::short_case => choice.cases.push(Case::ShortCase(ShortCase::parse(child))),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        choice
    }
}

#[derive(Debug, Clone)]
pub enum Case {
    LongCase(LongCase),
    ShortCase(ShortCase),
}

/// Case statement
#[derive(Debug, Clone, Default)]
pub struct LongCase {
    pub name: String,
    pub when: Option<When>,
    pub if_features: Vec<String>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub data_defs: Vec<DataDef>,
}

impl Parse for LongCase {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut case = LongCase::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => case.name = parse_string(child),
                Rule::when => case.when = Some(When::parse(child)),
                Rule::if_feature => case.if_features.push(parse_string(child)),
                Rule::status => case.status = Some(Status::parse(child)),
                Rule::description => case.description = Some(parse_string(child)),
                Rule::reference => case.reference = Some(parse_string(child)),
                Rule::data_def => case.data_defs.push(DataDef::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        case
    }
}

#[derive(Debug, Clone)]
pub enum ShortCase {
    Choice(Choice),
    Container(Container),
    Leaf(Leaf),
    LeafList(LeafList),
    List(List),
    Anydata(Anydata),
    Anyxml(Anyxml),
}

impl Parse for ShortCase {
    fn parse(pair: Pair<Rule>) -> Self {
        let node = pair.into_inner().next().expect("to always have inner node");

        match node.as_rule() {
            Rule::choice => ShortCase::Choice(Choice::parse(node)),
            Rule::container => ShortCase::Container(Container::parse(node)),
            Rule::leaf => ShortCase::Leaf(Leaf::parse(node)),
            Rule::leaf_list => ShortCase::LeafList(LeafList::parse(node)),
            Rule::list => ShortCase::List(List::parse(node)),
            Rule::anydata => ShortCase::Anydata(Anydata::parse(node)),
            Rule::anyxml => ShortCase::Anyxml(Anyxml::parse(node)),
            _ => unreachable!("Unexpected rule: {:?}", node.as_rule()),
        }
    }
}

/// Anydata statement
#[derive(Debug, Clone, Default)]
pub struct Anydata {
    pub name: String,
    pub when: Option<When>,
    pub if_features: Vec<String>,
    pub must: Vec<Must>,
    pub config: Option<bool>,
    pub mandatory: Option<bool>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for Anydata {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut anydata = Anydata::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => anydata.name = parse_string(child),
                Rule::when => anydata.when = Some(When::parse(child)),
                Rule::if_feature => anydata.if_features.push(parse_string(child)),
                Rule::must => anydata.must.push(Must::parse(child)),
                Rule::config => anydata.config = Some(parse_boolean(child)),
                Rule::mandatory => anydata.mandatory = Some(parse_boolean(child)),
                Rule::status => anydata.status = Some(Status::parse(child)),
                Rule::description => anydata.description = Some(parse_string(child)),
                Rule::reference => anydata.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        anydata
    }
}

/// Anyxml statement
#[derive(Debug, Clone, Default)]
pub struct Anyxml {
    pub name: String,
    pub when: Option<When>,
    pub if_features: Vec<String>,
    pub must: Vec<Must>,
    pub config: Option<bool>,
    pub mandatory: Option<bool>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for Anyxml {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut anyxml = Anyxml::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => anyxml.name = parse_string(child),
                Rule::when => anyxml.when = Some(When::parse(child)),
                Rule::if_feature => anyxml.if_features.push(parse_string(child)),
                Rule::must => anyxml.must.push(Must::parse(child)),
                Rule::config => anyxml.config = Some(parse_boolean(child)),
                Rule::mandatory => anyxml.mandatory = Some(parse_boolean(child)),
                Rule::status => anyxml.status = Some(Status::parse(child)),
                Rule::description => anyxml.description = Some(parse_string(child)),
                Rule::reference => anyxml.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        anyxml
    }
}

/// Uses statement
#[derive(Debug, Clone, Default)]
pub struct Uses {
    pub grouping: String,
    pub when: Option<When>,
    pub if_features: Vec<String>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub refines: Vec<Refine>,
    pub augments: Vec<Augment>,
}

impl Parse for Uses {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut uses = Uses::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => uses.grouping = parse_string(child),
                Rule::when => uses.when = Some(When::parse(child)),
                Rule::if_feature => uses.if_features.push(parse_string(child)),
                Rule::status => uses.status = Some(Status::parse(child)),
                Rule::description => uses.description = Some(parse_string(child)),
                Rule::reference => uses.reference = Some(parse_string(child)),
                Rule::refine => uses.refines.push(Refine::parse(child)),
                Rule::augment => uses.augments.push(Augment::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        uses
    }
}

/// Typedef statement
#[derive(Debug, Clone, Default)]
pub struct TypeDef {
    pub name: String,
    pub type_info: TypeInfo,
    pub units: Option<String>,
    pub default: Option<String>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for TypeDef {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut type_def = TypeDef::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => type_def.name = parse_string(child),
                Rule::type_info => type_def.type_info = TypeInfo::parse(child),
                Rule::units => type_def.units = Some(parse_string(child)),
                Rule::default => type_def.default = Some(parse_string(child)),
                Rule::status => type_def.status = Some(Status::parse(child)),
                Rule::description => type_def.description = Some(parse_string(child)),
                Rule::reference => type_def.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        type_def
    }
}

/// Type information
#[derive(Debug, Clone, Default)]
pub struct TypeInfo {
    pub name: String,
    pub type_body: Option<TypeBody>,
}

impl Parse for TypeInfo {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut type_info = TypeInfo::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => type_info.name = parse_string(child),
                Rule::numberical_restriction => type_info.type_body = Some(TypeBody::parse_numerical(child)),
                Rule::decimal64_specification => type_info.type_body = Some(TypeBody::parse_decimal(child)),
                Rule::string_restriction => type_info.type_body = Some(TypeBody::parse_string_restriction(child)),
                Rule::enum_specification => type_info.type_body = Some(TypeBody::parse_enum(child)),
                Rule::leafref_specification => type_info.type_body = Some(TypeBody::parse_leafref(child)),
                Rule::identityref_specification => type_info.type_body = Some(TypeBody::parse_identityref(child)),
                Rule::bits_specification => type_info.type_body = Some(TypeBody::parse_bit_specification(child)),
                Rule::binary_specification => type_info.type_body = Some(TypeBody::parse_binary_specification(child)),
                Rule::union_specification => type_info.type_body = Some(TypeBody::parse_union_specification(child)),
                Rule::instance_identifier_specification => {
                    type_info.type_body = Some(TypeBody::parse_instance_identifier(child))
                }
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        return type_info;
    }
}

/// Type body for specific type constraints
#[derive(Debug, Clone)]
pub enum TypeBody {
    Numerical {
        range: Range,
    },
    Decimal64 {
        fraction_digits: String,
        range: Option<Range>,
    },
    String {
        length: Option<Length>,
        patterns: Vec<Pattern>,
    },
    Enum {
        enums: Vec<EnumValue>,
    },
    Leafref {
        path: String,
        require_instance: Option<bool>,
    },
    Identityref {
        bases: Vec<String>,
    },
    InstanceIdentifier {
        require_instance: bool,
    },
    Bits {
        bits: Vec<Bit>,
    },
    Union {
        types: Vec<TypeInfo>,
    },
    Binary {
        length: Option<Length>,
    },
}

impl TypeBody {
    pub fn parse_numerical(input: Pair<Rule>) -> Self {
        TypeBody::Numerical {
            range: Range::parse(
                input
                    .into_inner()
                    .next()
                    .expect("numerical to always have range as the only child"),
            ),
        }
    }

    pub fn parse_decimal(input: Pair<Rule>) -> Self {
        let mut decimal_node = input.into_inner();
        let fractional_digits = parse_string(
            decimal_node
                .next()
                .expect("decimal's first child always to be a fractional_digits node"),
        );

        match decimal_node.next() {
            Some(range) => TypeBody::Decimal64 {
                fraction_digits: fractional_digits,
                range: Some(Range::parse(range)),
            },
            None => TypeBody::Decimal64 {
                fraction_digits: fractional_digits,
                range: None,
            },
        }
    }

    pub fn parse_string_restriction(input: Pair<Rule>) -> Self {
        let mut length = None;
        let mut patterns = Vec::new();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::length => length = Some(Length::parse(child)),
                Rule::pattern => patterns.push(Pattern::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        TypeBody::String { length, patterns }
    }

    pub fn parse_enum(input: Pair<Rule>) -> Self {
        let mut enums = Vec::new();
        for enum_child in input.into_inner() {
            enums.push(EnumValue::parse(enum_child));
        }

        TypeBody::Enum { enums }
    }

    pub fn parse_leafref(input: Pair<Rule>) -> Self {
        let mut leafref = input.into_inner();
        let path = parse_string(leafref.next().expect("first child of leafref to be the path"));

        match leafref.next() {
            Some(require_instance) => TypeBody::Leafref {
                path,
                require_instance: Some(parse_boolean(require_instance)),
            },
            None => TypeBody::Leafref {
                path,
                require_instance: None,
            },
        }
    }

    pub fn parse_identityref(input: Pair<Rule>) -> Self {
        let mut bases = Vec::new();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::base => bases.push(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        TypeBody::Identityref { bases }
    }

    pub fn parse_instance_identifier(input: Pair<Rule>) -> Self {
        TypeBody::InstanceIdentifier {
            require_instance: parse_boolean(
                input
                    .into_inner()
                    .next()
                    .expect("instance identifier to always have a require instance node"),
            ),
        }
    }

    pub fn parse_bit_specification(input: Pair<Rule>) -> Self {
        let mut bits = Vec::new();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::bit => bits.push(Bit::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        TypeBody::Bits { bits }
    }

    pub fn parse_binary_specification(input: Pair<Rule>) -> Self {
        match input.into_inner().next() {
            Some(length) => TypeBody::Binary {
                length: Some(Length::parse(length)),
            },
            None => TypeBody::Binary { length: None },
        }
    }

    pub fn parse_union_specification(input: Pair<Rule>) -> Self {
        let mut types = Vec::new();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::type_info => types.push(TypeInfo::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        TypeBody::Union { types }
    }
}

/// Range restriction
#[derive(Debug, Clone, Default)]
pub struct Range {
    pub value: String,
    pub error_message: Option<String>,
    pub error_app_tag: Option<String>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for Range {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut range = Range::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => range.value = parse_string(child),
                Rule::error_message => range.error_message = Some(parse_string(child)),
                Rule::error_app_tag => range.error_app_tag = Some(parse_string(child)),
                Rule::description => range.description = Some(parse_string(child)),
                Rule::reference => range.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        range
    }
}

/// Length restriction
#[derive(Debug, Clone, Default)]
pub struct Length {
    pub value: String,
    pub error_message: Option<String>,
    pub error_app_tag: Option<String>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for Length {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut length = Length::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => length.value = parse_string(child),
                Rule::error_message => length.error_message = Some(parse_string(child)),
                Rule::error_app_tag => length.error_app_tag = Some(parse_string(child)),
                Rule::description => length.description = Some(parse_string(child)),
                Rule::reference => length.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        length
    }
}

/// Pattern restriction
#[derive(Debug, Clone, Default)]
pub struct Pattern {
    pub value: String,
    pub modifier: Option<String>,
    pub error_message: Option<String>,
    pub error_app_tag: Option<String>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for Pattern {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut pattern = Pattern::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => pattern.value = parse_string(child),
                Rule::error_message => pattern.error_message = Some(parse_string(child)),
                Rule::error_app_tag => pattern.error_app_tag = Some(parse_string(child)),
                Rule::description => pattern.description = Some(parse_string(child)),
                Rule::reference => pattern.reference = Some(parse_string(child)),
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
}

/// Enum value
#[derive(Debug, Clone, Default)]
pub struct EnumValue {
    pub name: String,
    pub if_features: Vec<String>,
    pub value: Option<i64>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for EnumValue {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut enum_value = EnumValue::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => enum_value.name = parse_string(child),
                Rule::if_feature => enum_value.if_features.push(parse_string(child)),
                Rule::value => enum_value.value = Some(parse_integer(child)),
                Rule::status => enum_value.status = Some(Status::parse(child)),
                Rule::description => enum_value.description = Some(parse_string(child)),
                Rule::reference => enum_value.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        enum_value
    }
}

/// Bit value
#[derive(Debug, Clone, Default)]
pub struct Bit {
    pub name: String,
    pub if_features: Vec<String>,
    pub position: Option<i64>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for Bit {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut bit = Bit::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => bit.name = parse_string(child),
                Rule::if_feature => bit.if_features.push(parse_string(child)),
                Rule::position => bit.position = Some(parse_integer(child)),
                Rule::status => bit.status = Some(Status::parse(child)),
                Rule::description => bit.description = Some(parse_string(child)),
                Rule::reference => bit.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        bit
    }
}

/// Grouping statement
#[derive(Debug, Clone, Default)]
pub struct Grouping {
    pub name: String,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub type_defs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub data_defs: Vec<DataDef>,
    pub actions: Vec<Action>,
    pub notifications: Vec<Notification>,
}

impl Parse for Grouping {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut grouping = Grouping::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => grouping.name = parse_string(child),
                Rule::status => grouping.status = Some(Status::parse(child)),
                Rule::description => grouping.description = Some(parse_string(child)),
                Rule::reference => grouping.reference = Some(parse_string(child)),
                Rule::type_def => grouping.type_defs.push(TypeDef::parse(child)),
                Rule::grouping => grouping.groupings.push(Grouping::parse(child)),
                Rule::data_def => grouping.data_defs.push(DataDef::parse(child)),
                Rule::action => grouping.actions.push(Action::parse(child)),
                Rule::notification => grouping.notifications.push(Notification::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        grouping
    }
}

/// Extension statement
#[derive(Debug, Clone, Default)]
pub struct Extension {
    pub name: String,
    pub argument: Option<Argument>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for Extension {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut extension = Extension::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => extension.name = parse_string(child),
                Rule::argument => extension.argument = Some(Argument::parse(child)),
                Rule::status => extension.status = Some(Status::parse(child)),
                Rule::description => extension.description = Some(parse_string(child)),
                Rule::reference => extension.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        extension
    }
}

/// Argument for extension
#[derive(Debug, Clone, Default)]
pub struct Argument {
    pub name: String,
    pub yin_element: Option<bool>,
}

impl Parse for Argument {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut argument = Argument::default();
        let mut input = pair.into_inner();

        argument.name = parse_string(input.next().expect("first child to always be the name"));
        if let Some(yin_element) = input.next() {
            argument.yin_element = Some(parse_boolean(yin_element))
        }

        argument
    }
}

/// Feature statement
#[derive(Debug, Clone, Default)]
pub struct Feature {
    pub name: String,
    pub if_features: Vec<String>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for Feature {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut feature = Feature::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => feature.name = parse_string(child),
                Rule::if_feature => feature.if_features.push(parse_string(child)),
                Rule::status => feature.status = Some(Status::parse(child)),
                Rule::description => feature.description = Some(parse_string(child)),
                Rule::reference => feature.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        feature
    }
}

/// Identity statement
#[derive(Debug, Clone, Default)]
pub struct Identity {
    pub name: String,
    pub if_features: Vec<String>,
    pub bases: Vec<String>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for Identity {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut identity = Identity::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => identity.name = parse_string(child),
                Rule::if_feature => identity.if_features.push(parse_string(child)),
                Rule::base => identity.bases.push(parse_string(child)),
                Rule::status => identity.status = Some(Status::parse(child)),
                Rule::description => identity.description = Some(parse_string(child)),
                Rule::reference => identity.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        identity
    }
}

/// Augment statement
#[derive(Debug, Clone, Default)]
pub struct Augment {
    pub target: String,
    pub when: Option<When>,
    pub if_features: Vec<String>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub data_defs: Vec<DataDef>,
    pub cases: Vec<Case>,
    pub actions: Vec<Action>,
    pub notifications: Vec<Notification>,
}

impl Parse for Augment {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut augment = Augment::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => augment.target = parse_string(child),
                Rule::when => augment.when = Some(When::parse(child)),
                Rule::if_feature => augment.if_features.push(parse_string(child)),
                Rule::status => augment.status = Some(Status::parse(child)),
                Rule::description => augment.description = Some(parse_string(child)),
                Rule::reference => augment.reference = Some(parse_string(child)),
                Rule::data_def => augment.data_defs.push(DataDef::parse(child)),
                Rule::long_case => augment.cases.push(Case::LongCase(LongCase::parse(child))),
                Rule::action => augment.actions.push(Action::parse(child)),
                Rule::notification => augment.notifications.push(Notification::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        augment
    }
}

/// RPC statement
#[derive(Debug, Clone, Default)]
pub struct Rpc {
    pub name: String,
    pub if_features: Vec<String>,
    pub must: Vec<Must>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub type_defs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub input: Option<Input>,
    pub output: Option<Output>,
}

impl Parse for Rpc {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut rpc = Rpc::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => rpc.name = parse_string(child),
                Rule::input => rpc.input = Some(Input::parse(child)),
                Rule::output => rpc.output = Some(Output::parse(child)),
                Rule::if_feature => rpc.if_features.push(parse_string(child)),
                Rule::must => rpc.must.push(Must::parse(child)),
                Rule::status => rpc.status = Some(Status::parse(child)),
                Rule::description => rpc.description = Some(parse_string(child)),
                Rule::reference => rpc.reference = Some(parse_string(child)),
                Rule::type_def => rpc.type_defs.push(TypeDef::parse(child)),
                Rule::grouping => rpc.groupings.push(Grouping::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        rpc
    }
}

/// Input statement
#[derive(Debug, Clone, Default)]
pub struct Input {
    pub must: Vec<Must>,
    pub type_defs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub data_defs: Vec<DataDef>,
}

impl Parse for Input {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut input = Input::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::must => input.must.push(Must::parse(child)),
                Rule::type_def => input.type_defs.push(TypeDef::parse(child)),
                Rule::grouping => input.groupings.push(Grouping::parse(child)),
                Rule::data_def => input.data_defs.push(DataDef::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        input
    }
}

/// Output statement
#[derive(Debug, Clone, Default)]
pub struct Output {
    pub must: Vec<Must>,
    pub type_defs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub data_defs: Vec<DataDef>,
}

impl Parse for Output {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut output = Output::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::must => output.must.push(Must::parse(child)),
                Rule::type_def => output.type_defs.push(TypeDef::parse(child)),
                Rule::grouping => output.groupings.push(Grouping::parse(child)),
                Rule::data_def => output.data_defs.push(DataDef::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        output
    }
}

/// Action statement
#[derive(Debug, Clone, Default)]
pub struct Action {
    pub name: String,
    pub if_features: Vec<String>,
    pub must: Vec<Must>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub type_defs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub input: Option<Input>,
    pub output: Option<Output>,
}

impl Parse for Action {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut action = Action::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => action.name = parse_string(child),
                Rule::input => action.input = Some(Input::parse(child)),
                Rule::output => action.output = Some(Output::parse(child)),
                Rule::if_feature => action.if_features.push(parse_string(child)),
                Rule::must => action.must.push(Must::parse(child)),
                Rule::status => action.status = Some(Status::parse(child)),
                Rule::description => action.description = Some(parse_string(child)),
                Rule::reference => action.reference = Some(parse_string(child)),
                Rule::type_def => action.type_defs.push(TypeDef::parse(child)),
                Rule::grouping => action.groupings.push(Grouping::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        action
    }
}

/// Notification statement
#[derive(Debug, Clone, Default)]
pub struct Notification {
    pub name: String,
    pub if_features: Vec<String>,
    pub must: Vec<Must>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub type_defs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub data_defs: Vec<DataDef>,
}

impl Parse for Notification {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut notification = Notification::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => notification.name = parse_string(child),
                Rule::data_def => notification.data_defs.push(DataDef::parse(child)),
                Rule::if_feature => notification.if_features.push(parse_string(child)),
                Rule::must => notification.must.push(Must::parse(child)),
                Rule::status => notification.status = Some(Status::parse(child)),
                Rule::description => notification.description = Some(parse_string(child)),
                Rule::reference => notification.reference = Some(parse_string(child)),
                Rule::type_def => notification.type_defs.push(TypeDef::parse(child)),
                Rule::grouping => notification.groupings.push(Grouping::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        notification
    }
}

/// Deviation statement
#[derive(Debug, Clone, Default)]
pub struct Deviation {
    pub target: String,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub not_supported: bool,
    pub add: Vec<DeviateAdd>,
    pub delete: Vec<DeviateDelete>,
    pub replace: Vec<DeviateReplace>,
}

impl Parse for Deviation {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut deviation = Deviation::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => deviation.target = parse_string(child),
                Rule::description => deviation.description = Some(parse_string(child)),
                Rule::reference => deviation.reference = Some(parse_string(child)),
                Rule::deviation_not_supported => deviation.not_supported = true,
                Rule::deviate_add => deviation.add.push(DeviateAdd::parse(child)),
                Rule::deviate_delete => deviation.delete.push(DeviateDelete::parse(child)),
                Rule::deviate_replace => deviation.replace.push(DeviateReplace::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        deviation
    }
}

/// Deviate add
#[derive(Debug, Clone, Default)]
pub struct DeviateAdd {
    pub target: String,
    pub units: Option<String>,
    pub must: Vec<Must>,
    pub unique: Vec<String>,
    pub default: Vec<String>,
    pub config: Option<bool>,
    pub mandatory: Option<bool>,
    pub min_elements: Option<i64>,
    pub max_elements: Option<MaxElements>,
}

impl Parse for DeviateAdd {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut deviate = DeviateAdd::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => deviate.target = parse_string(child),
                Rule::units => deviate.units = Some(parse_string(child)),
                Rule::must => deviate.must.push(Must::parse(child)),
                Rule::unique => deviate.unique.push(parse_string(child)),
                Rule::default => deviate.default.push(parse_string(child)),
                Rule::config => deviate.config = Some(parse_boolean(child)),
                Rule::mandatory => deviate.mandatory = Some(parse_boolean(child)),
                Rule::min_elements => deviate.min_elements = Some(parse_integer(child)),
                Rule::max_elements => deviate.max_elements = Some(MaxElements::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        deviate
    }
}

/// Deviate delete
#[derive(Debug, Clone, Default)]
pub struct DeviateDelete {
    pub target: String,
    pub units: Option<String>,
    pub must: Vec<Must>,
    pub unique: Vec<String>,
    pub default: Vec<String>,
}

impl Parse for DeviateDelete {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut deviate = DeviateDelete::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => deviate.target = parse_string(child),
                Rule::units => deviate.units = Some(parse_string(child)),
                Rule::default => deviate.default.push(parse_string(child)),
                Rule::must => deviate.must.push(Must::parse(child)),
                Rule::unique => deviate.unique.push(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        deviate
    }
}

/// Deviate replace
#[derive(Debug, Clone, Default)]
pub struct DeviateReplace {
    pub target: String,
    pub type_info: Option<TypeInfo>,
    pub units: Option<String>,
    pub default: Vec<String>,
    pub config: Option<bool>,
    pub mandatory: Option<bool>,
    pub min_elements: Option<i64>,
    pub max_elements: Option<MaxElements>,
}

impl Parse for DeviateReplace {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut deviate = DeviateReplace::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => deviate.target = parse_string(child),
                Rule::type_info => deviate.type_info = Some(TypeInfo::parse(child)),
                Rule::units => deviate.units = Some(parse_string(child)),
                Rule::default => deviate.default.push(parse_string(child)),
                Rule::config => deviate.config = Some(parse_boolean(child)),
                Rule::mandatory => deviate.mandatory = Some(parse_boolean(child)),
                Rule::min_elements => deviate.min_elements = Some(parse_integer(child)),
                Rule::max_elements => deviate.max_elements = Some(MaxElements::parse(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        deviate
    }
}

/// Refine statement
#[derive(Debug, Clone, Default)]
pub struct Refine {
    pub target: String,
    pub if_features: Vec<String>,
    pub must: Vec<Must>,
    pub presence: Option<String>,
    pub default: Vec<String>,
    pub config: Option<bool>,
    pub mandatory: Option<bool>,
    pub min_elements: Option<i64>,
    pub max_elements: Option<MaxElements>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for Refine {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut refine = Refine::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => refine.target = parse_string(child),
                Rule::if_feature => refine.if_features.push(parse_string(child)),
                Rule::must => refine.must.push(Must::parse(child)),
                Rule::presence => refine.presence = Some(parse_string(child)),
                Rule::default => refine.default.push(parse_string(child)),
                Rule::config => refine.config = Some(parse_boolean(child)),
                Rule::mandatory => refine.mandatory = Some(parse_boolean(child)),
                Rule::min_elements => refine.min_elements = Some(parse_integer(child)),
                Rule::max_elements => refine.max_elements = Some(MaxElements::parse(child)),
                Rule::description => refine.description = Some(parse_string(child)),
                Rule::reference => refine.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        refine
    }
}

/// Must statement
#[derive(Debug, Clone, Default)]
pub struct Must {
    pub condition: String,
    pub error_message: Option<String>,
    pub error_app_tag: Option<String>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for Must {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut must = Must::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => must.condition = parse_string(child),
                Rule::error_message => must.error_message = Some(parse_string(child)),
                Rule::error_app_tag => must.error_app_tag = Some(parse_string(child)),
                Rule::description => must.description = Some(parse_string(child)),
                Rule::reference => must.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        must
    }
}

/// When statement
#[derive(Debug, Clone, Default)]
pub struct When {
    pub condition: String,
    pub description: Option<String>,
    pub reference: Option<String>,
}

impl Parse for When {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut when = When::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::string => when.condition = parse_string(child),
                Rule::description => when.description = Some(parse_string(child)),
                Rule::reference => when.reference = Some(parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        when
    }
}

/// Max elements value
#[derive(Debug, Clone, Default)]
pub enum MaxElements {
    #[default]
    Unbounded,
    Value(i64),
}

impl Parse for MaxElements {
    fn parse(pair: Pair<Rule>) -> Self {
        let max_elements = pair
            .into_inner()
            .next()
            .expect("max-elements to always have a max_elements_value as the only child");

        match max_elements.as_rule() {
            Rule::integer => MaxElements::Value(parse_integer(max_elements)),
            Rule::string => MaxElements::Unbounded,
            _ => unreachable!("Unexpected rule: {:?}", max_elements),
        }
    }
}

/// Ordered by value
#[derive(Debug, Clone, Default)]
pub enum OrderedBy {
    User,
    #[default]
    System,
}

impl Parse for OrderedBy {
    fn parse(pair: Pair<Rule>) -> Self {
        let ordered_by = pair
            .into_inner()
            .next()
            .expect("ordered-by to always have a ordered_by_value as the only child");

        match ordered_by.as_str() {
            "user" => OrderedBy::User,
            "system" => OrderedBy::System,
            _ => unreachable!("Unexpected ordered-by: {:?}", ordered_by),
        }
    }
}

/// Status value
#[derive(Debug, Clone, Default)]
pub enum Status {
    #[default]
    Current,
    Obsolete,
    Deprecated,
}

impl Parse for Status {
    fn parse(pair: Pair<Rule>) -> Self {
        let status = pair
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
}
