use dioxus::core::{DynamicNode, Template, TemplateNode};
use dioxus::prelude::*;

#[component]
pub fn PageTitle(children: Element) -> Element {
    let vnode = children?;
    let page_title = match vnode.template {
        Template {
            roots: &[TemplateNode::Text { text }],
            node_paths: &[],
            attr_paths: &[],
            ..
        } => text.to_string(),
        Template {
            roots: &[TemplateNode::Dynamic { id }],
            node_paths: &[&[0]],
            attr_paths: &[],
            ..
        } => {
            let node = &vnode.dynamic_nodes[id];
            match node {
                DynamicNode::Text(text) => text.value.clone(),
                _ => {
                    return rsx!();
                }
            }
        }
        _ => {
            return rsx!();
        }
    };

    rsx! {
        document::Title { "{page_title} | Mango³" }
    }
}
