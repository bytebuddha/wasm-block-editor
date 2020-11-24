use yew::prelude::*;
use wasm_block_editor::*;
use wasm_block_editor::components::{Editor, SimpleRenderer};

pub struct App {
    _link: ComponentLink<Self>,
    document: Document
}

pub enum Msg {

}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {
            _link,
            document: get_document()
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Editor<SimpleRenderer> document=self.document.clone() renderer=SimpleRenderer />
        }
    }
}

fn get_document() -> Document {
    Document {
        container_class: None,
        container_id: None,
        container_style: None,
        container_tag: None,
        blocks: get_blocks()
    }
}

fn get_blocks() -> Vec<Block> {
    vec![
        Block {
            container_id: None,
            container_tag: None,
            container_class: None,
            container_style: None,
            class: None,
            style: None,
            id: None,
            content: BlockContent::Heading { level: HeadingLevel::H1, text: "Page Title".into() }
        },
        Block {
            container_id: None,
            container_tag: None,
            container_class: None,
            container_style: None,
            class: None,
            style: None,
            id: None,
            content: BlockContent::Paragraph { text: "Some paragraph text here".into() }
        },
        Block {
            container_id: None,
            container_tag: None,
            container_class: None,
            container_style: None,
            class: None,
            style: None,
            id: None,
            content: BlockContent::List {
                style: ListStyle::Unordered,
                items: vec![
                    BlockContent::Label { text: "Unordered".into() },
                    BlockContent::Label { text: "List".into() },
                    BlockContent::Label { text: "Of".into() },
                    BlockContent::Label { text: "Items".into() },

                ]
            }
        },
        Block {
            container_id: None,
            container_tag: None,
            container_class: None,
            container_style: None,
            class: None,
            style: None,
            id: None,
            content: BlockContent::Code { text: "let var = 'c';".into() }
        },
        Block {
            container_id: None,
            container_tag: Some("div".into()),
            container_class: None,
            container_style: None,
            class: None,
            style: None,
            id: None,
            content: BlockContent::RawHtml { html: "<h4>Custom Html</h4>".into() }
        },
    ]
}