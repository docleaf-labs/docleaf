#[derive(Debug, PartialEq)]
pub struct DoxygenType {
    // Attributes
    version: DoxVersionNumber,
    // Children
    compound_def: Option<CompoundDefType>,
}

#[derive(Debug, PartialEq)]
pub struct CompoundDefType {
    // Attributes
    id: String,
    kind: DoxCompoundKind,
    language: DoxLanguage,
    prot: DoxProtectionKind,
    final_: bool,
    inline: bool,
    sealed: bool,
    abstract_: bool,
    // Children
    compoundname: String,
    title: Option<String>,
    basecompoundref: Vec<CompoundRefType>,
    derivedcompoundref: Vec<CompoundRefType>,
    includes: Vec<IncType>,
    includedby: Vec<IncType>,
    incdepgraph: Option<GraphType>,
    invincdepgraph: Option<GraphType>,
    innerdir: Vec<RefType>,
    innerfile: Vec<RefType>,
    innerclass: Vec<RefType>,
    innernamespace: Vec<RefType>,
    innerpage: Vec<RefType>,
    innergroup: Vec<RefType>,
    templateparamlist: Option<TemplateparamlistType>,
    sectiondef: Vec<SectionDefType>,
    tableofcontents: Option<Box<TableofcontentsType>>,
    requiresclause: Option<LinkedTextType>,
    initializer: Option<LinkedTextType>,
    briefdescription: Option<DescriptionType>,
    detaileddescription: Option<DescriptionType>,
    inheritancegraph: Option<GraphType>,
    collaborationgraph: Option<GraphType>,
    programlisting: Option<ListingType>,
    location: Option<LocationType>,
    listofallmembers: Option<ListofallmembersType>,
}

#[derive(Debug, PartialEq)]
pub struct ListofallmembersType {
    // Attributes

    // Children
    member: Vec<MemberRefType>,
}

#[derive(Debug, PartialEq)]
pub struct MemberRefType {
    // Attributes
    refid: String,
    prot: DoxProtectionKind,
    virt: DoxVirtualKind,
    ambiguityscope: String,
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
    kind: DoxSectionKind,
    // Children
    header: Option<String>,
    description: Option<DescriptionType>,
    memberdef: vec1::Vec1<MemberDefType>,
}

#[derive(Debug, PartialEq)]
pub struct MemberDefType {
    // Attributes
    kind: DoxMemberKind,
    id: String,
    prot: DoxProtectionKind,
    static_: bool,
    strong: bool,
    const_: bool,
    explicit: bool,
    inline: bool,
    refqual: DoxRefQualifierKind,
    virt: DoxVirtualKind,
    volatile: bool,
    mutable: bool,
    noexcept: bool,
    constexpr: bool,
    readable: bool,
    writable: bool,
    initonly: bool,
    settable: bool,
    privatesettable: bool,
    protectedsettable: bool,
    gettable: bool,
    privategettable: bool,
    protectedgettable: bool,
    final_: bool,
    sealed: bool,
    new: bool,
    add: bool,
    remove: bool,
    raise: bool,
    optional: bool,
    required: bool,
    accessor: DoxAccessor,
    attribute: bool,
    property: bool,
    readonly: bool,
    bound: bool,
    removable: bool,
    constrained: bool,
    transient: bool,
    maybevoid: bool,
    maybedefault: bool,
    maybeambiguous: bool,
    // Children
    templateparamlist: Option<TemplateparamlistType>,
    type_: Option<LinkedTextType>,
    reimplements: Vec<ReimplementType>,
    reimplementedby: Vec<ReimplementType>,
    param: Vec<ParamType>,
    enumvalue: Vec<EnumvalueType>,
    requiresclause: Option<LinkedTextType>,
    initializer: Option<LinkedTextType>,
    exceptions: Option<LinkedTextType>,
    briefdescription: Option<DescriptionType>,
    detaileddescription: Option<DescriptionType>,
    inbodydescription: Option<DescriptionType>,
    location: LocationType,
    references: Vec<ReferenceType>,
    referencedby: Vec<ReferenceType>,
}

#[derive(Debug, PartialEq)]
pub struct DescriptionType {
    // Attributes

    // Children
    title: Option<String>,
    para: Vec<DocParaType>,
    internal: Vec<DocInternalType>,
    sect1: Vec<DocSect1Type>,
}

#[derive(Debug, PartialEq)]
pub struct EnumvalueType {
    // Attributes
    id: String,
    prot: DoxProtectionKind,
    // Children
    initializer: Option<LinkedTextType>,
    briefdescription: Option<DescriptionType>,
    detaileddescription: Option<DescriptionType>,
}

#[derive(Debug, PartialEq)]
pub struct TemplateparamlistType {
    // Attributes

    // Children
    param: Vec<ParamType>,
}

#[derive(Debug, PartialEq)]
pub struct ParamType {
    // Attributes

    // Children
    type_: Option<LinkedTextType>,
    defval: Option<LinkedTextType>,
    typeconstraint: Option<LinkedTextType>,
    briefdescription: Option<DescriptionType>,
}

#[derive(Debug, PartialEq)]
pub struct LinkedTextType {
    // Attributes

    // Children
    ref_: Vec<RefTextType>,
}

#[derive(Debug, PartialEq)]
pub struct GraphType {
    // Attributes

    // Children
    node: vec1::Vec1<NodeType>,
}

#[derive(Debug, PartialEq)]
pub struct NodeType {
    // Attributes
    id: String,
    // Children
    link: Option<LinkType>,
    childnode: Vec<ChildnodeType>,
}

#[derive(Debug, PartialEq)]
pub struct ChildnodeType {
    // Attributes
    refid: String,
    relation: DoxGraphRelation,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct LinkType {
    // Attributes
    refid: String,
    external: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct ListingType {
    // Attributes
    filename: String,
    // Children
    codeline: Vec<CodelineType>,
}

#[derive(Debug, PartialEq)]
pub struct CodelineType {
    // Attributes
    lineno: i32,
    refid: String,
    refkind: DoxRefKind,
    external: bool,
    // Children
    highlight: Vec<HighlightType>,
}

#[derive(Debug, PartialEq)]
pub struct HighlightType {
    // Attributes
    class: DoxHighlightClass,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct SpType {
    // Attributes
    value: i32,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct ReferenceType {
    // Attributes
    refid: String,
    compoundref: String,
    startline: i32,
    endline: i32,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct LocationType {
    // Attributes
    file: String,
    line: i32,
    column: i32,
    declfile: String,
    declline: i32,
    declcolumn: i32,
    bodyfile: String,
    bodystart: i32,
    bodyend: i32,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocSect1Type {
    // Attributes
    id: String,
    // Children
    title: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct DocSect2Type {
    // Attributes
    id: String,
    // Children
    title: String,
}

#[derive(Debug, PartialEq)]
pub struct DocSect3Type {
    // Attributes
    id: String,
    // Children
    title: String,
}

#[derive(Debug, PartialEq)]
pub struct DocSect4Type {
    // Attributes
    id: String,
    // Children
    title: String,
}

#[derive(Debug, PartialEq)]
pub struct DocInternalType {
    // Attributes

    // Children
    para: Vec<DocParaType>,
    sect1: Vec<DocSect1Type>,
}

#[derive(Debug, PartialEq)]
pub struct DocInternalS1Type {
    // Attributes

    // Children
    para: Vec<DocParaType>,
    sect2: Vec<DocSect2Type>,
}

#[derive(Debug, PartialEq)]
pub struct DocInternalS2Type {
    // Attributes

    // Children
    para: Vec<DocParaType>,
    sect3: Vec<DocSect3Type>,
}

#[derive(Debug, PartialEq)]
pub struct DocInternalS3Type {
    // Attributes

    // Children
    para: Vec<DocParaType>,
    sect3: Vec<DocSect4Type>,
}

#[derive(Debug, PartialEq)]
pub struct DocInternalS4Type {
    // Attributes

    // Children
    para: Vec<DocParaType>,
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
    url: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocAnchorType {
    // Attributes
    id: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocFormulaType {
    // Attributes
    id: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocIndexEntryType {
    // Attributes

    // Children
    primaryie: String,
    secondaryie: String,
}

#[derive(Debug, PartialEq)]
pub struct DocListType {
    // Attributes
    type_: String,
    start: i32,
    // Children
    listitem: vec1::Vec1<DocListItemType>,
}

#[derive(Debug, PartialEq)]
pub struct DocListItemType {
    // Attributes
    value: i32,
    // Children
    para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocSimpleSectType {
    // Attributes
    kind: DoxSimpleSectKind,
    // Children
    title: Option<DocTitleType>,
}

#[derive(Debug, PartialEq)]
pub struct DocVarListEntryType {
    // Attributes

    // Children
    term: DocTitleType,
}

#[derive(Debug, PartialEq)]
pub struct DocVariableListType {
    // Attributes

    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocRefTextType {
    // Attributes
    refid: String,
    kindref: DoxRefKind,
    external: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocTableType {
    // Attributes
    rows: i32,
    cols: i32,
    width: String,
    // Children
    caption: Option<DocCaptionType>,
    row: Vec<DocRowType>,
}

#[derive(Debug, PartialEq)]
pub struct DocRowType {
    // Attributes

    // Children
    entry: Vec<DocEntryType>,
}

#[derive(Debug, PartialEq)]
pub struct DocEntryType {
    // Attributes
    thead: bool,
    colspan: i32,
    rowspan: i32,
    align: DoxAlign,
    valign: DoxVerticalAlign,
    width: String,
    class: String,
    // Children
    para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocCaptionType {
    // Attributes
    id: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocHeadingType {
    // Attributes
    level: i32,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocImageType {
    // Attributes
    type_: DoxImageKind,
    name: String,
    width: String,
    height: String,
    alt: String,
    inline: bool,
    caption: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocDotMscType {
    // Attributes
    name: String,
    width: String,
    height: String,
    caption: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocImageFileType {
    // Attributes
    name: String,
    width: String,
    height: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocPlantumlType {
    // Attributes
    name: String,
    width: String,
    height: String,
    caption: String,
    engine: DoxPlantumlEngine,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocTocItemType {
    // Attributes
    id: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub struct DocTocListType {
    // Attributes

    // Children
    tocitem: Vec<DocTocItemType>,
}

#[derive(Debug, PartialEq)]
pub struct DocLanguageType {
    // Attributes
    langid: String,
    // Children
    para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocParamListType {
    // Attributes
    kind: DoxParamListKind,
    // Children
    parameteritem: Vec<DocParamListItem>,
}

#[derive(Debug, PartialEq)]
pub struct DocParamListItem {
    // Attributes

    // Children
    parameternamelist: Vec<DocParamNameList>,
    parameterdescription: DescriptionType,
}

#[derive(Debug, PartialEq)]
pub struct DocParamNameList {
    // Attributes

    // Children
    parametertype: Vec<DocParamType>,
    parametername: Vec<DocParamName>,
}

#[derive(Debug, PartialEq)]
pub struct DocParamType {
    // Attributes

    // Children
    ref_: Option<RefTextType>,
}

#[derive(Debug, PartialEq)]
pub struct DocParamName {
    // Attributes
    direction: DoxParamDir,
    // Children
    ref_: Option<RefTextType>,
}

#[derive(Debug, PartialEq)]
pub struct DocXRefSectType {
    // Attributes
    id: String,
    // Children
    xreftitle: Vec<String>,
    xrefdescription: DescriptionType,
}

#[derive(Debug, PartialEq)]
pub struct DocCopyType {
    // Attributes
    link: String,
    // Children
    para: Vec<DocParaType>,
    sect1: Vec<DocSect1Type>,
    internal: Option<DocInternalType>,
}

#[derive(Debug, PartialEq)]
pub struct DocDetailsType {
    // Attributes

    // Children
    para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocBlockQuoteType {
    // Attributes

    // Children
    para: Vec<DocParaType>,
}

#[derive(Debug, PartialEq)]
pub struct DocParBlockType {
    // Attributes

    // Children
    para: Vec<DocParaType>,
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
    tocsect: vec1::Vec1<TableofcontentsKindType>,
}

#[derive(Debug, PartialEq)]
pub struct TableofcontentsKindType {
    // Attributes

    // Children
    name: String,
    reference: String,
    tableofcontents: Vec<Box<TableofcontentsType>>,
}

#[derive(Debug, PartialEq)]
pub struct DocEmojiType {
    // Attributes
    name: String,
    unicode: String,
    // Children
}

#[derive(Debug, PartialEq)]
pub enum bool {
    Yes,
    No,
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
