use yew::prelude::*;
use yewtil::NeqAssign;
use crate::{Document, Renderer};

pub struct Editor<R: Renderer<Html> + Clone + PartialEq + 'static> {
    selected_index: Option<usize>,
    document: Document,
    props: EditorProps<R>,
    link: ComponentLink<Self>
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditorProps<R: Renderer<Html> + Clone + PartialEq> {
    pub renderer: R,
    #[prop_or_default]
    pub document: Option<Document>
}

pub enum Msg {
    Selected(usize)
}

impl <R: Renderer<Html> + PartialEq + Clone + 'static>Component for Editor<R> {
    type Message = Msg;
    type Properties = EditorProps<R>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let document = props.document.clone().unwrap_or_default();
        Self { props, link, document, selected_index: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Selected(index) => self.selected_index = Some(index),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let res = self.props.neq_assign(props);
        if let Some(document) = self.props.document.clone() {
            self.document = document;
        }
        res
    }

    fn view(&self) -> Html {
        html! {
            <div class="wasm-block-editor">
                {
                    for self.document.blocks.iter().enumerate().map(|(dex, item)| {
                        if let Some(index) = self.selected_index {
                            if dex == index {
                                return html! {
                                    <div class="wasm-block-editor-block" style="background-color:#efefef;margin:5px;padding:10px;">
                                        <select>
                                            <option>{"Label"}</option>
                                            <option>{"Paragraph"}</option>
                                        </select>
                                        <div class="wasm-block-editor-block-display" style="background-color: white;margin: 5px;">
                                            { self.props.renderer.render_block(item) }
                                        </div>
                                    </div>
                                };
                            }
                        }
                        html! {
                            <div class="wasm-block-editor-block" onclick=self.link.callback(move |_| Msg::Selected(dex))>
                                { self.props.renderer.render_block(item) }
                            </div>
                        }
                    })
                }
            </div>
        }
    }
}