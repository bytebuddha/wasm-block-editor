use wasm_block_editor::*;
use yew::prelude::*;
use wasm_block_editor::components::{View, SimpleRenderer};

pub struct App {
    link: ComponentLink<Self>,
    document: Document
}

pub enum Msg {
    Change(ChangeData)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            document: get_document()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(data) => {
                match data {
                    ChangeData::Value(value) => {
                        if let Ok(document) = serde_json::from_str(&value) {
                            self.document = document;
                        }
                    },
                    _ => unreachable!()
                }
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <div style="width:100%;height:100%">
            <div id="col-1">
                <button>{"Reload"}</button>
                <textarea style="width:99%;height:96.8%" onchange=self.link.callback(Msg::Change)>
                  {
                      serde_json::to_string_pretty(&self.document).unwrap()
                  }
                </textarea>
            </div>
            <div id="col-2">
                <View<SimpleRenderer> document=self.document.clone() renderer=SimpleRenderer />
            </div>
          </div>
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