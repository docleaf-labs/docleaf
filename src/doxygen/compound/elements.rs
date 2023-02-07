#[derive(Debug, PartialEq)]
pub struct Root {
    pub compound_def: CompoundDef,
}

#[derive(Debug, PartialEq)]
pub struct CompoundDef {
    pub id: String,
    pub compound_name: String,
    pub brief_description: Description,
    pub detailed_description: Description,
    pub section_defs: Vec<SectionDef>,
}

#[derive(Debug, PartialEq)]
pub struct SectionDef {
    pub kind: String,
    pub member_defs: Vec<MemberDef>,
}

#[derive(Debug, PartialEq)]
pub struct MemberDef {
    pub id: String,
    pub name: String,
    pub kind: MemberDefKind,
    pub brief_description: Description,
    pub detailed_description: Description,
}

#[derive(Debug, PartialEq)]
pub enum MemberDefKind {
    Enum { values: Vec<EnumValue> },
    Function { params: Vec<Param> },
    Variable,
    Unknown(String),
}

impl MemberDefKind {
    pub fn name(&self) -> String {
        match self {
            Self::Enum { .. } => String::from("enum"),
            Self::Function { .. } => String::from("function"),
            Self::Variable => String::from("variable"),
            Self::Unknown(name) => name.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Param {
    pub type_: LinkedText,
    pub declname: String,
}

#[derive(Debug, PartialEq)]
pub struct EnumValue {
    pub name: String,
    pub initializer: String,
    pub brief_description: Description,
    pub detailed_description: Description,
}

#[derive(Debug, Default, PartialEq)]
pub struct Description {
    pub content: Vec<DescriptionType>,
}

#[derive(Debug, PartialEq)]
pub enum DescriptionType {
    Para(Vec<DocPara>),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub enum DocPara {
    ParameterList(DocParamList),
    SimpleSect(DocSimpleSect),
    Ref(RefText),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct DocParamList {
    // kind: DoxParamListKind,
    pub parameter_items: Vec<DocParamListItem>,
}

#[derive(Debug, PartialEq)]
pub struct DocParamListItem {
    pub parameter_name_list: Vec<DocParamNameList>,
    pub parameter_description: Description,
}

#[derive(Debug, PartialEq)]
pub struct DocParamNameList {
    pub parameter_type: Option<DocParamType>,
    pub parameter_name: Option<DocParamName>,
}

#[derive(Debug, PartialEq)]
pub enum DocParamType {
    Ref(RefText),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub enum DocParamName {
    Ref(RefText),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct DocSimpleSect {
    // title: String,
    pub paras: Vec<DocPara>,
}

#[derive(Debug, PartialEq)]
pub enum DoxParamListKind {
    Param,
    Retval,
    Exception,
    TemplateParam,
}

#[derive(Debug, PartialEq)]
pub enum LinkedText {
    Ref(RefText),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct RefText {
    pub id: String,
    pub text: String,
}
