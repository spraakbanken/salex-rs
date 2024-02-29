use html_parser::{Dom, Element, ElementVariant, Node};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Html {
    name: String,
    attributes: HashMap<String, Option<String>>,
    classes: Vec<String>,
    children: Vec<Node>,
}

impl Html {
    pub fn new() -> Self {
        Self {
            name: "html".into(),
            attributes: HashMap::new(),
            classes: Vec::new(),
            children: Vec::new(),
        }
    }
    pub fn with_name(name: String) -> Self {
        Self {
            name,
            attributes: HashMap::new(),
            classes: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn div() -> Self {
        Self::with_name("div".into())
    }

    pub fn span() -> Self {
        Self::with_name("span".into())
    }

    pub fn sup() -> Self {
        Self::with_name("sup".into())
    }

    pub fn build(self) -> Element {
        create_elem(self.name, self.attributes, self.classes, self.children)
    }

    pub fn class(mut self, classname: &str) -> Self {
        self.classes.push(classname.into());
        self
    }

    pub fn add_text(mut self, txt: String) -> Self {
        self.children.push(Node::Text(txt));
        self
    }
    pub fn text<S: Into<String>>(mut self, txt: S) -> Self {
        self.add_text(txt.into())
    }

    pub fn child(mut self, child: Element) -> Self {
        self.children.push(Node::Element(child));
        self
    }
}
pub fn create_elem(
    name: String,
    attributes: HashMap<String, Option<String>>,
    classes: Vec<String>,
    children: Vec<Node>,
) -> Element {
    Element {
        id: None,
        name,
        variant: ElementVariant::Normal,
        attributes,
        classes,
        children,
    }
}
pub fn create_div(classname: &str) -> Element {
    Element {
        id: None,
        name: "div".into(),
        variant: ElementVariant::Normal,
        attributes: HashMap::new(),
        classes: vec![classname.into()],
        children: Vec::new(),
    }
}

pub fn sub_span(elem: &mut Element, classname: &str, text: Option<String>) {
    let mut span = create_elem(
        "span".into(),
        HashMap::new(),
        vec![classname.into()],
        Vec::new(),
    );
    if let Some(text) = text {
        span.children.push(Node::Text(text));
    }
    elem.children.push(Node::Element(span));
}

pub fn to_string(elem: &Element) -> String {
    let tag = &elem.name;
    let mut s = String::new();
    elem_to_string_impl(&mut s, elem);
    s
}

fn elem_to_string_impl(s: &mut String, elem: &Element) {
    s.push_str("<");
    s.push_str(&elem.name);
    for class in &elem.classes {
        s.push_str(&format!(r#" class="{}""#, class));
    }
    s.push_str(">");
    for child in &elem.children {
        node_to_string_impl(s, child);
    }
    s.push_str(&format!("</{}>", elem.name));
}

fn node_to_string_impl(s: &mut String, node: &Node) {
    match node {
        Node::Element(elem) => elem_to_string_impl(s, elem),
        Node::Text(txt) => s.push_str(txt),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use html_parser::Dom;
    use std::error::Error;
    use std::fs;
    use std::io::Read;

    #[test]
    fn build_html() {
        let root = Html::new().build();

        assert_eq!(
            root,
            create_elem("html".into(), HashMap::new(), Vec::new(), Vec::new())
        );
    }

    #[test]
    fn build_div() {
        let mut root = Html::div();

        root = root.text("look");
        let elem = root.build();

        assert_eq!(
            elem,
            create_elem(
                "div".into(),
                HashMap::new(),
                Vec::new(),
                vec![Node::Text("look".into())]
            )
        );
    }
}
