#[derive(Debug, PartialEq)]
pub struct DoxygenType {
    // Attributes
    // pub version: DoxVersionNumber,
    // Children
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
    // Children
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
    // pub templateparamlist: Option<TemplateparamlistType>,
    pub section_defs: Vec<SectionDefType>,
    // pub tableofcontents: Option<Box<TableofcontentsType>>,
    // pub requiresclause: Option<LinkedTextType>,
    // pub initializer: Option<LinkedTextType>,
    pub brief_description: Option<DescriptionType>,
    pub detailed_description: Option<DescriptionType>,
    // pub inheritancegraph: Option<GraphType>,
    // pub collaborationgraph: Option<GraphType>,
    // pub programlisting: Option<ListingType>,
    // pub location: Option<LocationType>,
    // pub listofallmembers: Option<ListofallmembersType>
}

#[derive(Debug, PartialEq)]
pub struct ListofallmembersType {
    // Attributes

    // Children
    pub member: Vec<MemberRefType>,
}

#[derive(Debug, PartialEq)]
pub struct MemberRefType {
    // Attributes
    pub refid: String,
    pub prot: DoxProtectionKind,
    pub virt: DoxVirtualKind,
    pub ambiguityscope: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocHtmlOnlyType {
    // Attributes

    // Children
}

#[derive(Debug, PartialEq)]
pub struct CompoundRefType {
    // Attributes

    // Children
}

#[derive(Debug, PartialEq)]
pub struct ReimplementType {
    // Attributes

    // Children
}

#[derive(Debug, PartialEq)]
pub struct IncType {
    // Attributes

    // Children
}

#[derive(Debug, PartialEq)]
pub struct RefType {
    // Attributes

    // Children
}

#[derive(Debug, PartialEq)]
pub struct RefTextType {
    // Attributes

    // Children
}

#[derive(Debug, PartialEq)]
pub struct SectionDefType {
    // Attributes
    pub kind: DoxSectionKind,
    // Children
    pub header: Option<String>,
    pub description: Option<DescriptionType>,
    pub memberdef: vec1::Vec1<MemberDefType>,
}

#[derive(Debug, PartialEq)]
pub struct MemberDefType {
    // Attributes
    pub kind: DoxMemberKind,
    pub id: String,
    pub prot: DoxProtectionKind,
    pub static_: bool,
    pub strong: bool,
    pub const_: bool,
    pub explicit: bool,
    pub inline: bool,
    pub refqual: DoxRefQualifierKind,
    pub virt: DoxVirtualKind,
    pub volatile: bool,
    pub mutable: bool,
    pub noexcept: bool,
    pub constexpr: bool,
    pub readable: bool,
    pub writable: bool,
    pub initonly: bool,
    pub settable: bool,
    pub privatesettable: bool,
    pub protectedsettable: bool,
    pub gettable: bool,
    pub privategettable: bool,
    pub protectedgettable: bool,
    pub final_: bool,
    pub sealed: bool,
    pub new: bool,
    pub add: bool,
    pub remove: bool,
    pub raise: bool,
    pub optional: bool,
    pub required: bool,
    pub accessor: DoxAccessor,
    pub attribute: bool,
    pub property: bool,
    pub readonly: bool,
    pub bound: bool,
    pub removable: bool,
    pub constrained: bool,
    pub transient: bool,
    pub maybevoid: bool,
    pub maybedefault: bool,
    pub maybeambiguous: bool,
    // Children
    pub templateparamlist: Option<TemplateparamlistType>,
    pub type_: Option<LinkedTextType>,
    pub reimplements: Vec<ReimplementType>,
    pub reimplementedby: Vec<ReimplementType>,
    pub param: Vec<ParamType>,
    pub enumvalue: Vec<EnumvalueType>,
    pub requiresclause: Option<LinkedTextType>,
    pub initializer: Option<LinkedTextType>,
    pub exceptions: Option<LinkedTextType>,
    pub brief_description: Option<DescriptionType>,
    pub detailed_description: Option<DescriptionType>,
    pub inbodydescription: Option<DescriptionType>,
    pub location: LocationType,
    pub references: Vec<ReferenceType>,
    pub referencedby: Vec<ReferenceType>,
}

#[derive(Debug, PartialEq)]
pub struct DescriptionType {
    // Attributes

    // Children
    pub title: Option<String>,
    pub para: Vec<DocParaType>,
    pub internal: Vec<DocInternalType>,
    pub sect1: Vec<DocSect1Type>,
}

#[derive(Debug, PartialEq)]
pub struct EnumvalueType {
    // Attributes
    pub id: String,
    pub prot: DoxProtectionKind,
    // Children
    pub initializer: Option<LinkedTextType>,
    pub brief_description: Option<DescriptionType>,
    pub detailed_description: Option<DescriptionType>,
}

#[derive(Debug, PartialEq)]
pub struct TemplateparamlistType {
    // Attributes

    // Children
    pub param: Vec<ParamType>,
}

#[derive(Debug, PartialEq)]
pub struct ParamType {
    // Attributes

    // Children
    pub type_: Option<LinkedTextType>,
    pub defval: Option<LinkedTextType>,
    pub typeconstraint: Option<LinkedTextType>,
    pub brief_description: Option<DescriptionType>,
}

#[derive(Debug, PartialEq)]
pub struct LinkedTextType {
    // Attributes

    // Children
    pub ref_: Vec<RefTextType>,
}

#[derive(Debug, PartialEq)]
pub struct GraphType {
    // Attributes

    // Children
    pub node: vec1::Vec1<NodeType>,
}

#[derive(Debug, PartialEq)]
pub struct NodeType {
    // Attributes
    pub id: String,
    // Children
    pub link: Option<LinkType>,
    pub childnode: Vec<ChildnodeType>,
}

#[derive(Debug, PartialEq)]
pub struct ChildnodeType {
    // Attributes
    pub refid: String,
    pub relation: DoxGraphRelation,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct LinkType {
    // Attributes
    pub refid: String,
    pub external: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct ListingType {
    // Attributes
    pub filename: String,
    // Children
    pub codeline: Vec<CodelineType>,
}

#[derive(Debug, PartialEq)]
pub struct CodelineType {
    // Attributes
    pub lineno: i32,
    pub refid: String,
    pub refkind: DoxRefKind,
    pub external: bool,
    // Children
    pub highlight: Vec<HighlightType>,
}

#[derive(Debug, PartialEq)]
pub struct HighlightType {
    // Attributes
    pub class: DoxHighlightClass,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct SpType {
    // Attributes
    pub value: i32,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct ReferenceType {
    // Attributes
    pub refid: String,
    pub compoundref: String,
    pub startline: i32,
    pub endline: i32,
    // Children
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
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocSect1Type {
    // Attributes
    pub id: String,
    // Children
    pub title: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct DocSect2Type {
    // Attributes
    pub id: String,
    // Children
    pub title: String,
}

#[derive(Debug, PartialEq)]
pub struct DocSect3Type {
    // Attributes
    pub id: String,
    // Children
    pub title: String,
}

#[derive(Debug, PartialEq)]
pub struct DocSect4Type {
    // Attributes
    pub id: String,
    // Children
    pub title: String,
}

#[derive(Debug, PartialEq)]
pub struct DocInternalType {
    // Attributes

    // Children
    pub para: Vec<DocParaType>,
    pub sect1: Vec<DocSect1Type>,
}

#[derive(Debug, PartialEq)]
pub struct DocInternalS1Type {
    // Attributes

    // Children
    pub para: Vec<DocParaType>,
    pub sect2: Vec<DocSect2Type>,
}

#[derive(Debug, PartialEq)]
pub struct DocInternalS2Type {
    // Attributes

    // Children
    pub para: Vec<DocParaType>,
    pub sect3: Vec<DocSect3Type>,
}

#[derive(Debug, PartialEq)]
pub struct DocInternalS3Type {
    // Attributes

    // Children
    pub para: Vec<DocParaType>,
    pub sect3: Vec<DocSect4Type>,
}

#[derive(Debug, PartialEq)]
pub struct DocInternalS4Type {
    // Attributes

    // Children
    pub para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocTitleType {
    // Attributes

    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocParaType {
    // Attributes

    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocMarkupType {
    // Attributes

    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocURLLink {
    // Attributes
    pub url: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocAnchorType {
    // Attributes
    pub id: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocFormulaType {
    // Attributes
    pub id: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocIndexEntryType {
    // Attributes

    // Children
    pub primaryie: String,
    pub secondaryie: String,
}

#[derive(Debug, PartialEq)]
pub struct DocListType {
    // Attributes
    pub type_: String,
    pub start: i32,
    // Children
    pub listitem: vec1::Vec1<DocListItemType>,
}

#[derive(Debug, PartialEq)]
pub struct DocListItemType {
    // Attributes
    pub value: i32,
    // Children
    pub para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocSimpleSectType {
    // Attributes
    pub kind: DoxSimpleSectKind,
    // Children
    pub title: Option<DocTitleType>,
}

#[derive(Debug, PartialEq)]
pub struct DocVarListEntryType {
    // Attributes

    // Children
    pub term: DocTitleType,
}

#[derive(Debug, PartialEq)]
pub struct DocVariableListType {
    // Attributes

    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocRefTextType {
    // Attributes
    pub refid: String,
    pub kindref: DoxRefKind,
    pub external: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocTableType {
    // Attributes
    pub rows: i32,
    pub cols: i32,
    pub width: String,
    // Children
    pub caption: Option<DocCaptionType>,
    pub row: Vec<DocRowType>,
}

#[derive(Debug, PartialEq)]
pub struct DocRowType {
    // Attributes

    // Children
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
    // Children
    pub para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocCaptionType {
    // Attributes
    pub id: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocHeadingType {
    // Attributes
    pub level: i32,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocImageType {
    // Attributes
    pub type_: DoxImageKind,
    pub name: String,
    pub width: String,
    pub height: String,
    pub alt: String,
    pub inline: bool,
    pub caption: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocDotMscType {
    // Attributes
    pub name: String,
    pub width: String,
    pub height: String,
    pub caption: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocImageFileType {
    // Attributes
    pub name: String,
    pub width: String,
    pub height: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocPlantumlType {
    // Attributes
    pub name: String,
    pub width: String,
    pub height: String,
    pub caption: String,
    pub engine: DoxPlantumlEngine,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocTocItemType {
    // Attributes
    pub id: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocTocListType {
    // Attributes

    // Children
    pub tocitem: Vec<DocTocItemType>,
}

#[derive(Debug, PartialEq)]
pub struct DocLanguageType {
    // Attributes
    pub langid: String,
    // Children
    pub para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocParamListType {
    // Attributes
    pub kind: DoxParamListKind,
    // Children
    pub parameteritem: Vec<DocParamListItem>,
}

#[derive(Debug, PartialEq)]
pub struct DocParamListItem {
    // Attributes

    // Children
    pub parameternamelist: Vec<DocParamNameList>,
    pub parameterdescription: DescriptionType,
}

#[derive(Debug, PartialEq)]
pub struct DocParamNameList {
    // Attributes

    // Children
    pub parametertype: Vec<DocParamType>,
    pub parametername: Vec<DocParamName>,
}

#[derive(Debug, PartialEq)]
pub struct DocParamType {
    // Attributes

    // Children
    pub ref_: Option<RefTextType>,
}

#[derive(Debug, PartialEq)]
pub struct DocParamName {
    // Attributes
    pub direction: DoxParamDir,
    // Children
    pub ref_: Option<RefTextType>,
}

#[derive(Debug, PartialEq)]
pub struct DocXRefSectType {
    // Attributes
    pub id: String,
    // Children
    pub xreftitle: Vec<String>,
    pub xrefdescription: DescriptionType,
}

#[derive(Debug, PartialEq)]
pub struct DocCopyType {
    // Attributes
    pub link: String,
    // Children
    pub para: Vec<DocParaType>,
    pub sect1: Vec<DocSect1Type>,
    pub internal: Option<DocInternalType>,
}

#[derive(Debug, PartialEq)]
pub struct DocDetailsType {
    // Attributes

    // Children
    pub para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocBlockQuoteType {
    // Attributes

    // Children
    pub para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocParBlockType {
    // Attributes

    // Children
    pub para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocEmptyType {
    // Attributes

    // Children
}

#[derive(Debug, PartialEq)]
pub struct TableofcontentsType {
    // Attributes

    // Children
    pub tocsect: vec1::Vec1<TableofcontentsKindType>,
}

#[derive(Debug, PartialEq)]
pub struct TableofcontentsKindType {
    // Attributes

    // Children
    pub name: String,
    pub reference: String,
    pub tableofcontents: Vec<Box<TableofcontentsType>>,
}

#[derive(Debug, PartialEq)]
pub struct DocEmojiType {
    // Attributes
    pub name: String,
    pub unicode: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub enum DoxGraphRelation {
    Include,
    Usage,
    TemplateInstance,
    PublicInheritance,
    ProtectedInheritance,
    PrivateInheritance,
    TypeConstraint,
}

#[derive(Debug, PartialEq)]
pub enum DoxRefKind {
    Compound,
    Member,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum DoxProtectionKind {
    Public,
    Protected,
    Private,
    Package,
}

#[derive(Debug, PartialEq)]
pub enum DoxRefQualifierKind {
    Lvalue,
    Rvalue,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum DoxVirtualKind {
    NonVirtual,
    Virtual,
    PureVirtual,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum DoxImageKind {
    Html,
    Latex,
    Docbook,
    Rtf,
    Xml,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum DoxParamListKind {
    Param,
    Retval,
    Exception,
    Templateparam,
}

type DoxCharRange = String;

#[derive(Debug, PartialEq)]
pub enum DoxParamDir {
    In,
    Out,
    Inout,
}

#[derive(Debug, PartialEq)]
pub enum DoxAccessor {
    Retain,
    Copy,
    Assign,
    Weak,
    Strong,
    Unretained,
}

#[derive(Debug, PartialEq)]
pub enum DoxAlign {
    Left,
    Right,
    Center,
}

#[derive(Debug, PartialEq)]
pub enum DoxVerticalAlign {
    Bottom,
    Top,
    Middle,
}
