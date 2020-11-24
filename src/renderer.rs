use crate::{Document, Block};

pub trait Renderer<Html> {
    fn render(&self, doc: &Document) -> Html;

    fn render_block(&self, block: &Block) -> Html;
}