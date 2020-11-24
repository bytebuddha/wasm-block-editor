use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ListStyle {
    Ordered,
    Unordered
}

impl Default for ListStyle {
    fn default() -> ListStyle { ListStyle::Unordered }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TableHeader {
    pub label: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TableRow {
    pub cells: Vec<TableCell>
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TableCell {
    pub text: String
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum BlockContent {
    Code { text: String },
    Label { text: String },
    RawHtml { html: String },
    Paragraph { text: String },
    Heading { level: HeadingLevel, text: String },
    List { style: ListStyle, items: Vec<BlockContent> },
    Table {
        headers: Vec<TableHeader>,
        rows: Vec<TableRow>
    }
}

impl BlockContent {
    pub fn block_type(&self) -> BlockType {
        match self {
            BlockContent::Code { .. } => BlockType::Code,
            BlockContent::Label { .. } => BlockType::Label,
            BlockContent::RawHtml { .. } => BlockType::RawHTML,
            BlockContent::Paragraph { .. } => BlockType::Paragraph,
            BlockContent::Heading { .. } => BlockType::Heading,
            BlockContent::List { .. } => BlockType::List,
            BlockContent::Table { .. } => BlockType::Table
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Copy)]
pub enum BlockType {
    Code,
    Label,
    RawHTML,
    Paragraph,
    Heading,
    List,
    Table
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Block {
    pub container_id: Option<String>,
    pub container_tag: Option<String>,
    pub container_class: Option<String>,
    pub container_style: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub id: Option<String>,
    pub content: BlockContent
}

impl Block {

    pub fn block_type(&self) -> BlockType {
        self.content.block_type()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Document {
    pub container_id: Option<String>,
    pub container_tag: Option<String>,
    pub container_class: Option<String>,
    pub container_style: Option<String>,
    pub blocks: Vec<Block>
}