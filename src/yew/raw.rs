use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::web_sys;

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct RawHTMLProps {
    pub html: String,
}

pub struct RawHTML {
    props: RawHTMLProps,
}

impl Component for RawHTML {
    type Message = ();
    type Properties = RawHTMLProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(&self.props.html[..]);

        let node = web_sys::Node::from(div);
        let vnode = VNode::VRef(node);
        vnode
    }
}