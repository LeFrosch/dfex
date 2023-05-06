use super::{Node, Metadata};

pub struct Tree {
    pub root: Node,
}

impl Tree {
    pub fn iter_pre<F>(&self, mut f: F) where F: FnMut(&Node) {
        self.root.iter_pre(&mut f);
    }

    pub fn iter_pre_mut<F>(&mut self, mut f: F) where F: FnMut(&mut Node) {
        self.root.iter_pre_mut(&mut f);
    }

    pub fn iter_post_mut<F>(&mut self, mut f: F) where F: FnMut(&mut Node) {
        self.root.iter_post_mut(&mut f);
    }

    pub fn debug_graph(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str("digraph ast {\n");

        self.iter_pre(|node| {
            let id = format!("{:p}", node);

            match node {
                Node::Alternation(node) => {
                    node.metadata.write_debug_graph("Alternation", &id, &mut buffer);

                    for c in node.children.iter() {
                        buffer.push_str(&format!("\"{}\"->\"{:p}\"\n", id, c));
                    }
                }
                Node::Concatenation(node) => {
                    node.metadata.write_debug_graph("Concatenation", &id, &mut buffer);

                    for c in node.children.iter() {
                        buffer.push_str(&format!("\"{}\"->\"{:p}\"\n", id, c));
                    }
                }
                Node::Kleene(node) => {
                    node.metadata.write_debug_graph("Kleene", &id, &mut buffer);
                    buffer.push_str(&format!("\"{}\"->\"{:p}\"\n", id, node.child.as_ref()));
                }
                Node::Literal(node) => {
                    node.metadata.write_debug_graph(&format!("Literal[{}] {}", node.id, node.character), &id, &mut buffer);
                }
            };
        });

        buffer.push_str("}");
        buffer
    }
}

impl Metadata {
    fn write_debug_graph(&self, name: &str, id: &str, buffer: &mut String) {
        buffer.push_str(&format!("\"{}\"", id));
        buffer.push_str("[shape=\"rectangle\", label=<<table border=\"0\">");

        buffer.push_str(&format!("<tr><td>{}</td></tr>", name));
        buffer.push_str(&format!("<tr><td>empty: {}</td></tr>", self.empty));
        buffer.push_str(&format!("<tr><td>first: {:?}</td></tr>", self.first));
        buffer.push_str(&format!("<tr><td>last: {:?}</td></tr>", self.last));
        buffer.push_str(&format!("<tr><td>next: {:?}</td></tr>", self.next));

        buffer.push_str("</table>>]\n");
    }
}
