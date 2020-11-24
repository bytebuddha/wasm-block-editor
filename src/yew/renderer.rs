use yew::Html;
use yew::prelude::html;
use super::RawHTML;
use crate::{Document, Block, BlockContent, HeadingLevel, ListStyle, TableHeader, TableRow, Renderer};

#[derive(Default, PartialEq, Clone)]
pub struct SimpleRenderer;

impl SimpleRenderer {
    fn render_blocks<'a, B: Iterator<Item=&'a Block>>(&self, blocks: B) -> Html {
        html! {
            {
                for blocks.map(|item| {
                    Renderer::render_block(self, item)
                })
            }
        }
    }

    fn render_block_content(&self, content: &Block) -> Html {
        match &content.content {
            BlockContent::Label { text } => self.render_label_block(content, &text),
            BlockContent::Code { text } => self.render_code_block(content, &text),
            BlockContent::RawHtml { html } => html! { <RawHTML html=html /> },
            BlockContent::Paragraph { text } => self.render_paragraph_block(content, &text),
            BlockContent::Heading { text, level } => self.render_heading_block(content, level, &text),
            BlockContent::List { items , style } => self.render_list_block(content, style, items),
            BlockContent::Table { headers, rows } => self.render_table_block(content, headers, rows)
        }
    }

    fn render_label_block(&self, block: &Block, text: &str) -> Html {
        html! {
            <label
                id?=block.id.as_ref()
                class=block.class.as_ref()
                style?=block.style.as_ref()>
                    {text}
            </label>
        }
    }
    fn render_table_block(&self, block: &Block, headers: &Vec<TableHeader>, rows: &Vec<TableRow>) -> Html {
        html! {
            <table
                id?=block.id.as_ref()
                class=block.class.as_ref()
                style?=block.style.as_ref()>
              <thead>
                {
                    for headers.iter().map(|item| {
                        html! {
                            <th>{&item.label}</th>
                        }
                    })
                }
              </thead>
              <tbody>
                  {
                      for rows.iter().map(|item| {
                          html! {
                          <tr>
                              {
                                  for item.cells.iter().map(|item| {
                                      html! {
                                          <td>{&item.text}</td>
                                      }
                                  })
                              }
                          </tr>
                          }
                      })
                  }
              </tbody>
            </table>
        }
    }


    fn render_code_block(&self, block: &Block, text: &str) -> Html {
        html! {
            <code
                id?=block.id.as_ref()
                class=block.class.as_ref()
                style?=block.style.as_ref()>
                    {text}
            </code>
        }
    }

    fn render_paragraph_block(&self, block: &Block, text: &str) -> Html {
        html! {
            <p
                id?=block.id.as_ref()
                class=block.class.as_ref()
                style?=block.style.as_ref()>
                    {text}
            </p>
        }
    }

    fn render_heading_block(&self, block: &Block, level: &HeadingLevel, text: &str) -> Html {
        let tag = match level {
            HeadingLevel::H1 => "h1",
            HeadingLevel::H2 => "h2",
            HeadingLevel::H3 => "h3",
            HeadingLevel::H4 => "h4",
            HeadingLevel::H5 => "h5",
            HeadingLevel::H6 => "h6"
        };
        html! {
            <@{tag}
                id?=block.id.as_ref()
                class=block.class.as_ref()
                style?=block.style.as_ref()>
                    {text}
            </@>
           }
    }

    fn render_list_block(&self, block: &Block, style: &ListStyle, items: &Vec<BlockContent>) -> Html {
        let tag = match style {
            ListStyle::Unordered => "ul",
            ListStyle::Ordered => "ol"
        };
        html! {
            <@{tag}
                id?=block.id.as_ref()
                class=block.class.as_ref()
                style?=block.style.as_ref()>
                    {
                        for items.iter().map(|item| html! {
                            <li>{ self.render_block_content(&Block {
                                content: item.clone(),
                                container_tag: None,
                                container_id: None,
                                container_class: None,
                                container_style: None,
                                id: None,
                                style: None,
                                class: None
                              })
                            }</li>
                        })
                    }
            </@>
        }
    }
}

impl Renderer<Html> for SimpleRenderer {
    fn render(&self, doc: &Document) -> Html {
        if let Some(tag) = doc.container_tag.clone() {
            html! {
             <@{tag}
                 id?=doc.container_id.as_ref()
                 class=doc.container_class.as_ref()
                 style?=doc.container_style.as_ref()>
                 { self.render_blocks(doc.blocks.iter()) }
             </@>
           }
        } else {
            html! {
               { self.render_blocks(doc.blocks.iter()) }
           }
        }
    }

    fn render_block(&self, block: &Block) -> Html {
        if let Some(tag) = block.container_tag.clone() {
            html! {
             <@{tag}
                 id?=block.container_id.as_ref()
                 class=block.container_class.as_ref()
                 style?=block.container_style.as_ref()>
                 { self.render_block_content(block) }
             </@>
           }
        } else {
            html! {
               { self.render_block_content(block) }
           }
        }
    }
}