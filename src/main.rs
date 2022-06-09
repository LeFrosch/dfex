struct Metadata {
    empty: bool,
    first: Vec<usize>,
    next: Vec<usize>,
    last: Vec<usize>,
}

impl Metadata {
    fn new() -> Self {
        Metadata {
            empty: false,
            first: vec![],
            next: vec![],
            last: vec![],
        }
    }

    fn write_graph_fields(&self, name: &str, id: &str, buffer: &mut String) {
        buffer.push_str(&format!("\"{}\"", id));
        buffer.push_str("[shape=\"rectangle\", label=<<table border=\"0\">");

        buffer.push_str(&format!("<tr><td>{}</td></tr>", name));
        buffer.push_str(&format!("<tr><td>empty: {}</td></tr>", self.empty));
        buffer.push_str(&format!("<tr><td>first: {:?}</td></tr>", self.first));
        buffer.push_str(&format!("<tr><td>next: {:?}</td></tr>", self.next));

        buffer.push_str("</table>>]\n");
    }
}

struct AlternationNode {
    metadata: Metadata,
    children: Vec<Node>,
}

struct ConcatenationNode {
    metadata: Metadata,
    children: Vec<Node>,
}

struct KleeneNode {
    metadata: Metadata,
    child: Box<Node>,
}

struct LiteralNode {
    metadata: Metadata,
    id: usize,
    character: char,
}

enum Node {
    Alternation(AlternationNode),
    Concatenation(ConcatenationNode),
    Kleene(KleeneNode),
    Literal(LiteralNode),
}

impl Node {
    fn iter_children<F>(&mut self, f: &mut F) where F: FnMut(&mut Node) {
        match self {
            Node::Alternation(x) => {
                for c in x.children.iter_mut() {
                    c.iter_pre(f)
                }
            }
            Node::Concatenation(x) => {
                for c in x.children.iter_mut() {
                    c.iter_pre(f)
                }
            }
            Node::Kleene(x) => {
                x.child.iter_pre(f);
            }
            Node::Literal(_) => {}
        }
    }

    fn iter_pre<F>(&mut self, f: &mut F) where F: FnMut(&mut Node) {
        self.iter_children(f);
        f(self);
    }
}


struct Tree {
    root: Node,
}

impl Tree {
    fn iter_pre<F>(&mut self, mut f: F) where F: FnMut(&mut Node) {
        self.root.iter_pre(&mut f);
    }
}

fn main() {
    let root = Node::Concatenation(ConcatenationNode {
        metadata: Metadata::new(),
        children: vec![
            Node::Kleene(KleeneNode {
                metadata: Metadata::new(),
                child: Box::new(Node::Alternation(AlternationNode {
                    metadata: Metadata::new(),
                    children: vec![
                        Node::Literal(LiteralNode {
                            metadata: Metadata::new(),
                            id: 0,
                            character: 'a',
                        }),
                        Node::Literal(LiteralNode {
                            metadata: Metadata::new(),
                            id: 1,
                            character: 'b',
                        }),
                    ],
                })),
            }),
            Node::Concatenation(ConcatenationNode {
                metadata: Metadata::new(),
                children: vec![
                    Node::Literal(LiteralNode {
                        metadata: Metadata::new(),
                        id: 2,
                        character: 'a',
                    }),
                    Node::Alternation(AlternationNode {
                        metadata: Metadata::new(),
                        children: vec![
                            Node::Literal(LiteralNode {
                                metadata: Metadata::new(),
                                id: 3,
                                character: 'a',
                            }),
                            Node::Literal(LiteralNode {
                                metadata: Metadata::new(),
                                id: 4,
                                character: 'b',
                            }),
                        ],
                    }),
                ],
            }),
        ],
    });

    let mut tree = Tree { root };

    let mut buffer = String::new();
    buffer.push_str("digraph ast {\n");

    tree.iter_pre(|node| {
        let id = format!("{:p}", node);

        match node {
            Node::Alternation(node) => {
                node.metadata.write_graph_fields("Alternation", &id, &mut buffer);

                for c in node.children.iter() {
                    buffer.push_str(&format!("\"{}\"->\"{:p}\"\n", id, c));
                }
            }
            Node::Concatenation(node) => {
                node.metadata.write_graph_fields("Concatenation", &id, &mut buffer);

                for c in node.children.iter() {
                    buffer.push_str(&format!("\"{}\"->\"{:p}\"\n", id, c));
                }
            }
            Node::Kleene(node) => {
                node.metadata.write_graph_fields("Kleene", &id, &mut buffer);
                buffer.push_str(&format!("\"{}\"->\"{:p}\"\n", id, node.child.as_ref()));
            }
            Node::Literal(node) => {
                node.metadata.write_graph_fields(&format!("Literal[{}] {}", node.id, node.character), &id, &mut buffer);
            }
        };
    });

    buffer.push_str("}");

    println!("{}", buffer)
}