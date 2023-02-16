//! Automatically generated by:
//!
//!    generate.py examples/nutshell/xml/compound.xsd src/doxygen/compound/elements.rs
//!

#[derive(Debug, PartialEq)]
pub struct DoxygenType {
    // Attributes
    // pub version: DoxVersionNumber,
    // Elements
    pub compound_def: Option<CompoundDefType>,
}

#[derive(Debug, PartialEq)]
pub struct CompoundDefType {
    // Attributes
    pub id: String,
    // pub kind: DoxCompoundKind,
    // pub language: DoxLanguage,
    // pub prot: DoxProtectionKind,
    // pub final_: bool,
    // pub inline: bool,
    // pub sealed: bool,
    // pub abstract_: bool,
    // Elements
    pub compound_name: String,
    // pub title: Option<String>,
    // pub basecompoundref: Vec<CompoundRefType>,
    // pub derivedcompoundref: Vec<CompoundRefType>,
    // pub includes: Vec<IncType>,
    // pub includedby: Vec<IncType>,
    // pub incdepgraph: Option<GraphType>,
    // pub invincdepgraph: Option<GraphType>,
    // pub innerdir: Vec<RefType>,
    // pub innerfile: Vec<RefType>,
    // pub innerclass: Vec<RefType>,
    // pub innernamespace: Vec<RefType>,
    // pub innerpage: Vec<RefType>,
    // pub innergroup: Vec<RefType>,
    // pub templateparamlist: Option<TemplateParamListType>,
    pub section_defs: Vec<SectionDefType>,
    // pub tableofcontents: Option<TableOfContentsType>,
    // pub requiresclause: Option<LinkedTextType>,
    // pub initializer: Option<LinkedTextType>,
    pub brief_description: Option<DescriptionType>,
    pub detailed_description: Option<DescriptionType>,
    // pub inheritancegraph: Option<GraphType>,
    // pub collaborationgraph: Option<GraphType>,
    // pub programlisting: Option<ListingType>,
    // pub location: Option<LocationType>,
    // pub listofallmembers: Option<ListofallmembersType>,
}

#[derive(Debug, PartialEq)]
pub struct ListofallmembersType {
    // Attributes

    // Elements
    pub member: Vec<MemberRefType>,
}

#[derive(Debug, PartialEq)]
pub struct MemberRefType {
    // Attributes
    // pub ref_id: String,
    pub prot: DoxProtectionKind,
    pub virt: DoxVirtualKind,
    pub ambiguityscope: String,
    // Elements
    // pub scope: String,
    // pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct DocHtmlOnlyType {
    // Attributes
    // pub block: String,
    // Content
    content: String,
}

#[derive(Debug, PartialEq)]
pub struct CompoundRefType {
    // Attributes
    // pub ref_id: String,
    // pub prot: DoxProtectionKind,
    // pub virt: DoxVirtualKind,
    // Content
    content: String,
}

#[derive(Debug, PartialEq)]
pub struct ReimplementType {
    // Attributes
    // pub ref_id: String,
    // Content
    content: String,
}

#[derive(Debug, PartialEq)]
pub struct IncType {
    // Attributes
    // pub ref_id: String,
    // pub local: bool,
    // Content
    content: String,
}

#[derive(Debug, PartialEq)]
pub struct RefType {
    // Attributes
    // pub ref_id: String,
    // pub prot: DoxProtectionKind,
    // pub inline: bool,
    // Content
    content: String,
}

#[derive(Debug, PartialEq)]
pub struct RefTextType {
    // Attributes
    pub ref_id: String,
    // pub kindref: DoxRefKind,
    // pub external: String,
    // pub tooltip: String,
    // Content
    content: String,
}

#[derive(Debug, PartialEq)]
pub struct SectionDefType {
    // Attributes
    pub kind: DoxSectionKind,
    // Elements
    // pub header: Option<String>,
    // pub description: Option<DescriptionType>,
    pub member_defs: vec1::Vec1<MemberDefType>,
}

#[derive(Debug, PartialEq)]
pub struct MemberDefType {
    // Attributes
    pub kind: DoxMemberKind,
    pub id: String,
    // pub prot: DoxProtectionKind,
    // pub static_: bool,
    // pub strong: bool,
    // pub const_: bool,
    // pub explicit: bool,
    // pub inline: bool,
    // pub refqual: DoxRefQualifierKind,
    // pub virt: DoxVirtualKind,
    // pub volatile: bool,
    // pub mutable: bool,
    // pub noexcept: bool,
    // pub constexpr: bool,
    // pub readable: bool,
    // pub writable: bool,
    // pub initonly: bool,
    // pub settable: bool,
    // pub privatesettable: bool,
    // pub protectedsettable: bool,
    // pub gettable: bool,
    // pub privategettable: bool,
    // pub protectedgettable: bool,
    // pub final_: bool,
    // pub sealed: bool,
    // pub new: bool,
    // pub add: bool,
    // pub remove: bool,
    // pub raise: bool,
    // pub optional: bool,
    // pub required: bool,
    // pub accessor: DoxAccessor,
    // pub attribute: bool,
    // pub property: bool,
    // pub readonly: bool,
    // pub bound: bool,
    // pub removable: bool,
    // pub constrained: bool,
    // pub transient: bool,
    // pub maybevoid: bool,
    // pub maybedefault: bool,
    // pub maybeambiguous: bool,
    // Elements
    // pub templateparamlist: Option<TemplateParamListType>,
    // pub type_: Option<LinkedTextType>,
    // pub definition: Option<String>,
    // pub argsstring: Option<String>,
    pub name: String,
    // pub qualifiedname: Option<String>,
    // pub read: Option<String>,
    // pub write: Option<String>,
    // pub bitfield: Option<String>,
    // pub reimplements: Vec<ReimplementType>,
    // pub reimplementedby: Vec<ReimplementType>,
    // pub param: Vec<ParamType>,
    // pub enumvalue: Vec<EnumValueType>,
    // pub requiresclause: Option<LinkedTextType>,
    // pub initializer: Option<LinkedTextType>,
    // pub exceptions: Option<LinkedTextType>,
    pub brief_description: Option<DescriptionType>,
    pub detailed_description: Option<DescriptionType>,
    // pub inbodydescription: Option<DescriptionType>,
    // pub location: LocationType,
    // pub references: Vec<ReferenceType>,
    // pub referencedby: Vec<ReferenceType>,
}

#[derive(Debug, PartialEq)]
pub struct DescriptionType {
    // Attributes

    // Contents
    content: Vec<DescriptionTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DescriptionTypeItem {
    Title(String),
    Para(DocParaType),
    Internal(DocInternalType),
    Sect1(DocSect1Type),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct EnumValueType {
    // Attributes
    // pub id: String,
    // pub prot: DoxProtectionKind,
    // Elements
    pub name: String,
    pub initializer: Option<LinkedTextType>,
    pub brief_description: Option<DescriptionType>,
    pub detailed_description: Option<DescriptionType>,
}

#[derive(Debug, PartialEq)]
pub struct TemplateParamListType {
    // Attributes

    // Elements
    pub param: Vec<ParamType>,
}

#[derive(Debug, PartialEq)]
pub struct ParamType {
    // Attributes

    // Elements
    // pub attributes: Option<String>,
    pub type_: Option<LinkedTextType>,
    pub declname: Option<String>,
    // pub defname: Option<String>,
    // pub array: Option<String>,
    // pub defval: Option<LinkedTextType>,
    // pub typeconstraint: Option<LinkedTextType>,
    // pub brief_description: Option<DescriptionType>,
}

#[derive(Debug, PartialEq)]
pub struct LinkedTextType {
    // Attributes

    // Contents
    content: Vec<LinkedTextTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum LinkedTextTypeItem {
    Ref(RefTextType),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct GraphType {
    // Attributes

    // Elements
    pub node: vec1::Vec1<NodeType>,
}

#[derive(Debug, PartialEq)]
pub struct NodeType {
    // Attributes
    pub id: String,
    // Elements
    // pub label: String,
    pub link: Option<LinkType>,
    pub childnode: Vec<ChildnodeType>,
}

#[derive(Debug, PartialEq)]
pub struct ChildnodeType {
    // Attributes
    // pub ref_id: String,
    pub relation: DoxGraphRelation,
    // Elements
    // pub edgelabel: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct LinkType {
    // Attributes
    // pub ref_id: String,
    pub external: String,
    // Elements
}

#[derive(Debug, PartialEq)]
pub struct ListingType {
    // Attributes
    pub filename: String,
    // Elements
    pub codeline: Vec<CodelineType>,
}

#[derive(Debug, PartialEq)]
pub struct CodelineType {
    // Attributes
    pub lineno: i32,
    // pub ref_id: String,
    pub refkind: DoxRefKind,
    pub external: bool,
    // Elements
    pub highlight: Vec<HighlightType>,
}

#[derive(Debug, PartialEq)]
pub struct HighlightType {
    // Attributes
    // pub class: DoxHighlightClass,
    // Contents
    content: Vec<HighlightTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum HighlightTypeItem {
    Sp(SpType),
    Ref(RefTextType),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct SpType {
    // Attributes
    // pub value: i32,
    // Contents
    content: String,
}

#[derive(Debug, PartialEq)]
pub struct ReferenceType {
    // Attributes
    // pub ref_id: String,
    // pub compoundref: String,
    // pub startline: i32,
    // pub endline: i32,
    // Contents
    content: String,
}

#[derive(Debug, PartialEq)]
pub struct LocationType {
    // Attributes
    pub file: String,
    pub line: i32,
    pub column: i32,
    pub declfile: String,
    pub declline: i32,
    pub declcolumn: i32,
    pub bodyfile: String,
    pub bodystart: i32,
    pub bodyend: i32,
    // Elements
}

#[derive(Debug, PartialEq)]
pub struct DocSect1Type {
    // Attributes
    // pub id: String,
    // Contents
    content: Vec<DocSect1TypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocSect1TypeItem {
    Title(String),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct DocSect2Type {
    // Attributes
    // pub id: String,
    // Contents
    content: Vec<DocSect2TypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocSect2TypeItem {
    Title(String),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct DocSect3Type {
    // Attributes
    // pub id: String,
    // Contents
    content: Vec<DocSect3TypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocSect3TypeItem {
    Title(String),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct DocSect4Type {
    // Attributes
    // pub id: String,
    // Contents
    content: Vec<DocSect4TypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocSect4TypeItem {
    Title(String),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct DocInternalType {
    // Attributes

    // Contents
    content: Vec<DocInternalTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocInternalTypeItem {
    Para(DocParaType),
    Sect1(DocSect1Type),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct DocInternalS1Type {
    // Attributes

    // Contents
    content: Vec<DocInternalS1TypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocInternalS1TypeItem {
    Para(DocParaType),
    Sect2(DocSect2Type),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct DocInternalS2Type {
    // Attributes

    // Contents
    content: Vec<DocInternalS2TypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocInternalS2TypeItem {
    Para(DocParaType),
    Sect3(DocSect3Type),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct DocInternalS3Type {
    // Attributes

    // Contents
    content: Vec<DocInternalS3TypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocInternalS3TypeItem {
    Para(DocParaType),
    Sect3(DocSect4Type),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct DocInternalS4Type {
    // Attributes

    // Contents
    content: Vec<DocInternalS4TypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocInternalS4TypeItem {
    Para(DocParaType),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct DocTitleType {
    // Attributes

    // Contents
    content: Vec<DocTitleTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocTitleTypeItem {
    DocTitleCmdGroup,
}

#[derive(Debug, PartialEq)]
pub struct DocParaType {
    // Attributes

    // Contents
    content: Vec<DocParaTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocParaTypeItem {
    DocCmdGroup,
}

#[derive(Debug, PartialEq)]
pub struct DocMarkupType {
    // Attributes

    // Contents
    content: Vec<DocMarkupTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocMarkupTypeItem {
    DocCmdGroup,
}

#[derive(Debug, PartialEq)]
pub struct DocURLLink {
    // Attributes
    // pub url: String,
    // Contents
    content: Vec<DocURLLinkItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocURLLinkItem {
    DocTitleCmdGroup,
}

#[derive(Debug, PartialEq)]
pub struct DocAnchorType {
    // Attributes
    // pub id: String,
    // Contents
    content: String,
}

#[derive(Debug, PartialEq)]
pub struct DocFormulaType {
    // Attributes
    // pub id: String,
    // Contents
    content: String,
}

#[derive(Debug, PartialEq)]
pub struct DocIndexEntryType {
    // Attributes

    // Elements
    pub primaryie: String,
    pub secondaryie: String,
}

#[derive(Debug, PartialEq)]
pub struct DocListType {
    // Attributes
    pub type_: String,
    pub start: i32,
    // Elements
    pub listitem: vec1::Vec1<DocListItemType>,
}

#[derive(Debug, PartialEq)]
pub struct DocListItemType {
    // Attributes
    pub value: i32,
    // Elements
    pub para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocSimpleSectType {
    // Attributes
    pub kind: DoxSimpleSectKind,
    // Elements
    pub title: Option<DocTitleType>,
}

#[derive(Debug, PartialEq)]
pub struct DocVarListEntryType {
    // Attributes

    // Elements
    pub term: DocTitleType,
}

#[derive(Debug, PartialEq)]
pub struct DocVariableListType {
    // Attributes

    // Elements
}

#[derive(Debug, PartialEq)]
pub struct DocRefTextType {
    // Attributes
    // pub ref_id: String,
    // pub kindref: DoxRefKind,
    // pub external: String,
    // Contents
    content: Vec<DocRefTextTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocRefTextTypeItem {
    DocTitleCmdGroup,
}

#[derive(Debug, PartialEq)]
pub struct DocTableType {
    // Attributes
    pub rows: i32,
    pub cols: i32,
    pub width: String,
    // Elements
    pub caption: Option<DocCaptionType>,
    pub row: Vec<DocRowType>,
}

#[derive(Debug, PartialEq)]
pub struct DocRowType {
    // Attributes

    // Elements
    pub entry: Vec<DocEntryType>,
}

#[derive(Debug, PartialEq)]
pub struct DocEntryType {
    // Attributes
    pub thead: bool,
    pub colspan: i32,
    pub rowspan: i32,
    pub align: DoxAlign,
    pub valign: DoxVerticalAlign,
    pub width: String,
    pub class: String,
    // Elements
    pub para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocCaptionType {
    // Attributes
    // pub id: String,
    // Contents
    content: Vec<DocCaptionTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocCaptionTypeItem {
    DocTitleCmdGroup,
}

#[derive(Debug, PartialEq)]
pub struct DocHeadingType {
    // Attributes
    // pub level: i32,
    // Contents
    content: Vec<DocHeadingTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocHeadingTypeItem {
    DocTitleCmdGroup,
}

#[derive(Debug, PartialEq)]
pub struct DocImageType {
    // Attributes
    // pub type_: DoxImageKind,
    // pub name: String,
    // pub width: String,
    // pub height: String,
    // pub alt: String,
    // pub inline: bool,
    // pub caption: String,
    // Contents
    content: Vec<DocImageTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocImageTypeItem {
    DocTitleCmdGroup,
}

#[derive(Debug, PartialEq)]
pub struct DocDotMscType {
    // Attributes
    // pub name: String,
    // pub width: String,
    // pub height: String,
    // pub caption: String,
    // Contents
    content: Vec<DocDotMscTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocDotMscTypeItem {
    DocTitleCmdGroup,
}

#[derive(Debug, PartialEq)]
pub struct DocImageFileType {
    // Attributes
    // pub name: String,
    // pub width: String,
    // pub height: String,
    // Contents
    content: Vec<DocImageFileTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocImageFileTypeItem {
    DocTitleCmdGroup,
}

#[derive(Debug, PartialEq)]
pub struct DocPlantumlType {
    // Attributes
    // pub name: String,
    // pub width: String,
    // pub height: String,
    // pub caption: String,
    // pub engine: DoxPlantumlEngine,
    // Contents
    content: Vec<DocPlantumlTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocPlantumlTypeItem {
    DocTitleCmdGroup,
}

#[derive(Debug, PartialEq)]
pub struct DocTocItemType {
    // Attributes
    // pub id: String,
    // Contents
    content: Vec<DocTocItemTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocTocItemTypeItem {
    DocTitleCmdGroup,
}

#[derive(Debug, PartialEq)]
pub struct DocTocListType {
    // Attributes

    // Elements
    pub tocitem: Vec<DocTocItemType>,
}

#[derive(Debug, PartialEq)]
pub struct DocLanguageType {
    // Attributes
    pub langid: String,
    // Elements
    pub para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocParamListType {
    // Attributes
    pub kind: DoxParamListKind,
    // Elements
    pub parameteritem: Vec<DocParamListItem>,
}

#[derive(Debug, PartialEq)]
pub struct DocParamListItem {
    // Attributes

    // Elements
    pub parameternamelist: Vec<DocParamNameList>,
    pub parameterdescription: DescriptionType,
}

#[derive(Debug, PartialEq)]
pub struct DocParamNameList {
    // Attributes

    // Elements
    pub parametertype: Vec<DocParamType>,
    pub parametername: Vec<DocParamName>,
}

#[derive(Debug, PartialEq)]
pub struct DocParamType {
    // Attributes

    // Contents
    content: Vec<DocParamTypeItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocParamTypeItem {
    Ref(RefTextType),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct DocParamName {
    // Attributes
    // pub direction: DoxParamDir,
    // Contents
    content: Vec<DocParamNameItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocParamNameItem {
    Ref(RefTextType),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct DocXRefSectType {
    // Attributes
    pub id: String,
    // Elements
    pub xreftitle: Vec<String>,
    pub xrefdescription: DescriptionType,
}

#[derive(Debug, PartialEq)]
pub struct DocCopyType {
    // Attributes
    pub link: String,
    // Elements
    pub para: Vec<DocParaType>,
    pub sect1: Vec<DocSect1Type>,
    pub internal: Option<DocInternalType>,
}

#[derive(Debug, PartialEq)]
pub struct DocDetailsType {
    // Attributes

    // Elements
    pub para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocBlockQuoteType {
    // Attributes

    // Elements
    pub para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocParBlockType {
    // Attributes

    // Elements
    pub para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocEmptyType {
    // Attributes

    // Elements
}

#[derive(Debug, PartialEq)]
pub struct TableOfContentsType {
    // Attributes

    // Elements
    pub tocsect: vec1::Vec1<TableOfContentsKindType>,
}

#[derive(Debug, PartialEq)]
pub struct TableOfContentsKindType {
    // Attributes

    // Elements
    pub name: String,
    pub reference: String,
    pub tableofcontents: Vec<TableOfContentsType>,
}

#[derive(Debug, PartialEq)]
pub struct DocEmojiType {
    // Attributes
    pub name: String,
    pub unicode: String,
    // Elements
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxGraphRelation {
    Include,
    Usage,
    TemplateInstance,
    PublicInheritance,
    ProtectedInheritance,
    PrivateInheritance,
    TypeConstraint,
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxRefKind {
    Compound,
    Member,
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxMemberKind {
    Define,
    Property,
    Event,
    Variable,
    Typedef,
    Enum,
    Function,
    Signal,
    Prototype,
    Friend,
    Dcop,
    Slot,
    Interface,
    Service,
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxProtectionKind {
    Public,
    Protected,
    Private,
    Package,
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxRefQualifierKind {
    Lvalue,
    Rvalue,
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxLanguage {
    Unknown,
    IDL,
    Java,
    CSharp,
    D,
    PHP,
    ObjectiveC,
    CPlusPlus,
    JavaScript,
    Python,
    Fortran,
    VHDL,
    XML,
    SQL,
    Markdown,
    Slice,
    Lex,
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxVirtualKind {
    NonVirtual,
    Virtual,
    PureVirtual,
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxCompoundKind {
    Class,
    Struct,
    Union,
    Interface,
    Protocol,
    Category,
    Exception,
    Service,
    Singleton,
    Module,
    Type,
    File,
    Namespace,
    Group,
    Page,
    Example,
    Dir,
    Concept,
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxSectionKind {
    UserDefined,
    PublicType,
    PublicFunc,
    PublicAttrib,
    PublicSlot,
    Signal,
    DcopFunc,
    Property,
    Event,
    PublicStaticFunc,
    PublicStaticAttrib,
    ProtectedType,
    ProtectedFunc,
    ProtectedAttrib,
    ProtectedSlot,
    ProtectedStaticFunc,
    ProtectedStaticAttrib,
    PackageType,
    PackageFunc,
    PackageAttrib,
    PackageStaticFunc,
    PackageStaticAttrib,
    PrivateType,
    PrivateFunc,
    PrivateAttrib,
    PrivateSlot,
    PrivateStaticFunc,
    PrivateStaticAttrib,
    Friend,
    Related,
    Define,
    Prototype,
    Typedef,
    Enum,
    Func,
    Var,
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxHighlightClass {
    Comment,
    Normal,
    Preprocessor,
    Keyword,
    Keywordtype,
    Keywordflow,
    Stringliteral,
    Charliteral,
    Vhdlkeyword,
    Vhdllogic,
    Vhdlchar,
    Vhdldigit,
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxSimpleSectKind {
    See,
    Return,
    Author,
    Authors,
    Version,
    Since,
    Date,
    Note,
    Warning,
    Pre,
    Post,
    Copyright,
    Invariant,
    Remark,
    Attention,
    Par,
    Rcs,
}

type DoxVersionNumber = String;

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxImageKind {
    Html,
    Latex,
    Docbook,
    Rtf,
    Xml,
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxPlantumlEngine {
    Uml,
    Bpm,
    Wire,
    Dot,
    Ditaa,
    Salt,
    Math,
    Latex,
    Gantt,
    Mindmap,
    Wbs,
    Yaml,
    Creole,
    Json,
    Flow,
    Board,
    Git,
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxParamListKind {
    Param,
    Retval,
    Exception,
    Templateparam,
}

type DoxCharRange = String;

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxParamDir {
    In,
    Out,
    Inout,
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxAccessor {
    Retain,
    Copy,
    Assign,
    Weak,
    Strong,
    Unretained,
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxAlign {
    Left,
    Right,
    Center,
}

#[derive(Debug, strum::EnumString, PartialEq)]
pub enum DoxVerticalAlign {
    Bottom,
    Top,
    Middle,
}