/// Represents a complete YANG module or submodule
#[derive(Debug, Clone)]
pub enum YangFile {
    Module(Module),
    Submodule(Submodule),
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

/// Represents a YANG submodule
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct BelongsTo {
    pub module: String,
    pub prefix: String,
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

/// Include statement
#[derive(Debug, Clone, Default)]
pub struct Include {
    pub module: String,
    pub revision_date: Option<String>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// Meta information for modules
#[derive(Debug, Clone, Default)]
pub struct MetaInfo {
    pub organization: Option<String>,
    pub contact: Option<String>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// Revision history
#[derive(Debug, Clone, Default)]
pub struct Revision {
    pub date: String,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// All possible schema nodes that can appear in a YANG module body
#[derive(Debug, Clone)]
pub enum SchemaNode {
    Container(Container),
    Leaf(Leaf),
    LeafList(LeafList),
    List(List),
    Choice(Choice),
    Anydata(Anydata),
    Anyxml(Anyxml),
    Uses(Uses),
    TypeDef(TypeDef),
    Grouping(Grouping),
    Extension(Extension),
    Feature(Feature),
    Identity(Identity),
    Augment(Augment),
    Rpc(Rpc),
    Notification(Notification),
    Deviation(Deviation),
}

/// Container statement
#[derive(Debug, Clone)]
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
    pub typedefs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub children: Vec<SchemaNode>,
    pub actions: Vec<Action>,
    pub notifications: Vec<Notification>,
}

/// Leaf statement
#[derive(Debug, Clone)]
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

/// Leaf-list statement
#[derive(Debug, Clone)]
pub struct LeafList {
    pub name: String,
    pub when: Option<When>,
    pub if_features: Vec<String>,
    pub type_info: TypeInfo,
    pub units: Option<String>,
    pub must: Vec<Must>,
    pub default: Vec<String>,
    pub config: Option<bool>,
    pub min_elements: Option<u32>,
    pub max_elements: Option<MaxElements>,
    pub ordered_by: Option<OrderedBy>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// List statement
#[derive(Debug, Clone)]
pub struct List {
    pub name: String,
    pub when: Option<When>,
    pub if_features: Vec<String>,
    pub must: Vec<Must>,
    pub key: Option<String>,
    pub unique: Vec<String>,
    pub config: Option<bool>,
    pub min_elements: Option<u32>,
    pub max_elements: Option<MaxElements>,
    pub ordered_by: Option<OrderedBy>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub typedefs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub children: Vec<SchemaNode>,
    pub actions: Vec<Action>,
    pub notifications: Vec<Notification>,
}

/// Choice statement
#[derive(Debug, Clone)]
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

/// Case statement
#[derive(Debug, Clone)]
pub struct Case {
    pub name: String,
    pub when: Option<When>,
    pub if_features: Vec<String>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub children: Vec<SchemaNode>,
}

/// Anydata statement
#[derive(Debug, Clone)]
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

/// Anyxml statement
#[derive(Debug, Clone)]
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

/// Uses statement
#[derive(Debug, Clone)]
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

/// Typedef statement
#[derive(Debug, Clone)]
pub struct TypeDef {
    pub name: String,
    pub type_info: TypeInfo,
    pub units: Option<String>,
    pub default: Option<String>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// Type information
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub type_body: Option<TypeBody>,
}

/// Type body for specific type constraints
#[derive(Debug, Clone)]
pub enum TypeBody {
    Numerical {
        range: Option<Range>,
    },
    Decimal64 {
        fraction_digits: u8,
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
        require_instance: Option<bool>,
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

/// Range restriction
#[derive(Debug, Clone)]
pub struct Range {
    pub value: String,
    pub error_message: Option<String>,
    pub error_app_tag: Option<String>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// Length restriction
#[derive(Debug, Clone)]
pub struct Length {
    pub value: String,
    pub error_message: Option<String>,
    pub error_app_tag: Option<String>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// Pattern restriction
#[derive(Debug, Clone)]
pub struct Pattern {
    pub value: String,
    pub modifier: Option<String>,
    pub error_message: Option<String>,
    pub error_app_tag: Option<String>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// Enum value
#[derive(Debug, Clone)]
pub struct EnumValue {
    pub name: String,
    pub if_features: Vec<String>,
    pub value: Option<i32>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// Bit value
#[derive(Debug, Clone)]
pub struct Bit {
    pub name: String,
    pub if_features: Vec<String>,
    pub position: Option<u32>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// Grouping statement
#[derive(Debug, Clone)]
pub struct Grouping {
    pub name: String,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub typedefs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub children: Vec<SchemaNode>,
    pub actions: Vec<Action>,
    pub notifications: Vec<Notification>,
}

/// Extension statement
#[derive(Debug, Clone)]
pub struct Extension {
    pub name: String,
    pub argument: Option<Argument>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// Argument for extension
#[derive(Debug, Clone)]
pub struct Argument {
    pub name: String,
    pub yin_element: Option<bool>,
}

/// Feature statement
#[derive(Debug, Clone)]
pub struct Feature {
    pub name: String,
    pub if_features: Vec<String>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// Identity statement
#[derive(Debug, Clone)]
pub struct Identity {
    pub name: String,
    pub if_features: Vec<String>,
    pub bases: Vec<String>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// Augment statement
#[derive(Debug, Clone)]
pub struct Augment {
    pub target: String,
    pub when: Option<When>,
    pub if_features: Vec<String>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub children: Vec<SchemaNode>,
    pub cases: Vec<Case>,
    pub actions: Vec<Action>,
    pub notifications: Vec<Notification>,
}

/// RPC statement
#[derive(Debug, Clone)]
pub struct Rpc {
    pub name: String,
    pub if_features: Vec<String>,
    pub must: Vec<Must>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub typedefs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub input: Option<Input>,
    pub output: Option<Output>,
}

/// Input statement
#[derive(Debug, Clone)]
pub struct Input {
    pub must: Vec<Must>,
    pub typedefs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub children: Vec<SchemaNode>,
}

/// Output statement
#[derive(Debug, Clone)]
pub struct Output {
    pub must: Vec<Must>,
    pub typedefs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub children: Vec<SchemaNode>,
}

/// Action statement
#[derive(Debug, Clone)]
pub struct Action {
    pub name: String,
    pub if_features: Vec<String>,
    pub must: Vec<Must>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub typedefs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub input: Option<Input>,
    pub output: Option<Output>,
}

/// Notification statement
#[derive(Debug, Clone)]
pub struct Notification {
    pub name: String,
    pub if_features: Vec<String>,
    pub must: Vec<Must>,
    pub status: Option<Status>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub typedefs: Vec<TypeDef>,
    pub groupings: Vec<Grouping>,
    pub children: Vec<SchemaNode>,
}

/// Deviation statement
#[derive(Debug, Clone)]
pub struct Deviation {
    pub target: String,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub not_supported: bool,
    pub add: Option<DeviateAdd>,
    pub delete: Option<DeviateDelete>,
    pub replace: Option<DeviateReplace>,
}

/// Deviate add
#[derive(Debug, Clone)]
pub struct DeviateAdd {
    pub units: Option<String>,
    pub must: Vec<Must>,
    pub unique: Vec<String>,
    pub default: Vec<String>,
    pub config: Option<bool>,
    pub mandatory: Option<bool>,
    pub min_elements: Option<u32>,
    pub max_elements: Option<MaxElements>,
}

/// Deviate delete
#[derive(Debug, Clone)]
pub struct DeviateDelete {
    pub units: Option<String>,
    pub must: Vec<Must>,
    pub unique: Vec<String>,
    pub default: Vec<String>,
}

/// Deviate replace
#[derive(Debug, Clone)]
pub struct DeviateReplace {
    pub type_info: Option<TypeInfo>,
    pub units: Option<String>,
    pub default: Vec<String>,
    pub config: Option<bool>,
    pub mandatory: Option<bool>,
    pub min_elements: Option<u32>,
    pub max_elements: Option<MaxElements>,
}

/// Refine statement
#[derive(Debug, Clone)]
pub struct Refine {
    pub target: String,
    pub if_features: Vec<String>,
    pub must: Vec<Must>,
    pub presence: Option<String>,
    pub default: Vec<String>,
    pub config: Option<bool>,
    pub mandatory: Option<bool>,
    pub min_elements: Option<u32>,
    pub max_elements: Option<MaxElements>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// Must statement
#[derive(Debug, Clone)]
pub struct Must {
    pub condition: String,
    pub error_message: Option<String>,
    pub error_app_tag: Option<String>,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// When statement
#[derive(Debug, Clone)]
pub struct When {
    pub condition: String,
    pub description: Option<String>,
    pub reference: Option<String>,
}

/// Max elements value
#[derive(Debug, Clone)]
pub enum MaxElements {
    Unbounded,
    Value(u32),
}

/// Ordered by value
#[derive(Debug, Clone)]
pub enum OrderedBy {
    User,
    System,
}

/// Status value
#[derive(Debug, Clone)]
pub enum Status {
    Current,
    Obsolete,
    Deprecated,
}
