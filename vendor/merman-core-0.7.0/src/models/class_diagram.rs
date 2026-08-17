use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassDiagram {
    #[serde(rename = "type")]
    pub diagram_type: String,
    pub direction: String,
    #[serde(rename = "accTitle")]
    #[serde(default)]
    pub acc_title: Option<String>,
    #[serde(rename = "accDescr")]
    #[serde(default)]
    pub acc_descr: Option<String>,
    pub classes: IndexMap<String, ClassNode>,
    #[serde(default)]
    pub relations: Vec<ClassRelation>,
    #[serde(default)]
    pub notes: Vec<ClassNote>,
    #[serde(default)]
    pub interfaces: Vec<ClassInterface>,
    #[serde(default)]
    pub namespaces: IndexMap<String, Namespace>,
    #[serde(rename = "styleClasses")]
    #[serde(default)]
    pub style_classes: IndexMap<String, StyleClass>,
    pub constants: ClassConstants,
}

impl ClassDiagram {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassNode {
    pub id: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub type_param: String,
    pub label: String,
    pub text: String,
    #[serde(rename = "cssClasses")]
    #[serde(default)]
    pub css_classes: String,
    #[serde(default)]
    pub methods: Vec<ClassMember>,
    #[serde(default)]
    pub members: Vec<ClassMember>,
    #[serde(default)]
    pub annotations: Vec<String>,
    #[serde(default)]
    pub styles: Vec<String>,
    #[serde(rename = "domId")]
    pub dom_id: String,
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
    #[serde(rename = "linkTarget")]
    #[serde(default)]
    pub link_target: Option<String>,
    #[serde(default)]
    pub tooltip: Option<String>,
    #[serde(rename = "haveCallback")]
    #[serde(default)]
    pub have_callback: bool,
    #[serde(default)]
    pub callback: Option<Map<String, Value>>,
    #[serde(rename = "callbackEffective")]
    #[serde(default)]
    pub callback_effective: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassMember {
    #[serde(rename = "memberType")]
    pub member_type: String,
    pub visibility: String,
    pub id: String,
    pub classifier: String,
    pub parameters: String,
    #[serde(rename = "returnType")]
    pub return_type: String,
    #[serde(rename = "displayText")]
    pub display_text: String,
    #[serde(rename = "cssStyle")]
    pub css_style: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassRelation {
    pub id: String,
    pub id1: String,
    pub id2: String,
    #[serde(rename = "relationTitle1")]
    pub relation_title_1: String,
    #[serde(rename = "relationTitle2")]
    pub relation_title_2: String,
    pub title: String,
    pub relation: RelationShape,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationShape {
    pub type1: i32,
    pub type2: i32,
    #[serde(rename = "lineType")]
    pub line_type: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassNote {
    pub id: String,
    #[serde(rename = "class")]
    pub class_id: Option<String>,
    pub text: String,
    #[serde(default)]
    pub parent: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassInterface {
    pub id: String,
    pub label: String,
    #[serde(rename = "classId")]
    pub class_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Namespace {
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "domId")]
    pub dom_id: String,
    #[serde(rename = "classIds")]
    #[serde(default)]
    pub class_ids: Vec<String>,
    #[serde(rename = "noteIds")]
    #[serde(default)]
    pub note_ids: Vec<String>,
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(default)]
    pub explicit: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleClass {
    pub id: String,
    #[serde(default)]
    pub styles: Vec<String>,
    #[serde(rename = "textStyles")]
    #[serde(default)]
    pub text_styles: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassConstants {
    #[serde(rename = "lineType")]
    pub line_type: ClassLineTypeConstants,
    #[serde(rename = "relationType")]
    pub relation_type: ClassRelationTypeConstants,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassLineTypeConstants {
    pub line: i32,
    #[serde(rename = "dottedLine")]
    pub dotted_line: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassRelationTypeConstants {
    pub none: i32,
    pub aggregation: i32,
    pub extension: i32,
    pub composition: i32,
    pub dependency: i32,
    pub lollipop: i32,
}
