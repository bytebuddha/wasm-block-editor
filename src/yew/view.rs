use yew::prelude::*;
use yewtil::NeqAssign;
use crate::{Document, Renderer};

pub struct View<R: Renderer<Html> + Clone + 'static> {
    props: ViewProps<R>,
    _link: ComponentLink<Self>
}


#[derive(Debug, Properties, Clone)]
pub struct ViewProps<R: Renderer<Html> + Clone> {
    pub document: Document,
    pub renderer: R
}

impl <R: Renderer<Html> + Clone + 'static>PartialEq for ViewProps<R> {
    fn eq(&self, other: &Self) -> bool {
        self.document == other.document
    }
}

impl <R: Renderer<Html> + Clone + 'static>Component for View<R> {
    type Message = ();
    type Properties = ViewProps<R>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props, _link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        self.props.renderer.render(&self.props.document)
    }
}