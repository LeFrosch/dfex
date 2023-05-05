mod ast;

fn main() {
    let root = ast::Node::Concatenation(ast::ConcatenationNode {
        metadata: ast::Metadata::new(),
        children: vec![
            ast::Node::Kleene(ast::KleeneNode {
                metadata: ast::Metadata::new(),
                child: Box::new(ast::Node::Alternation(ast::AlternationNode {
                    metadata: ast::Metadata::new(),
                    children: vec![
                        ast::Node::Literal(ast::LiteralNode {
                            metadata: ast::Metadata::new(),
                            id: 0,
                            character: 'a',
                        }),
                        ast::Node::Literal(ast::LiteralNode {
                            metadata: ast::Metadata::new(),
                            id: 1,
                            character: 'b',
                        }),
                    ],
                })),
            }),
            ast::Node::Concatenation(ast::ConcatenationNode {
                metadata: ast::Metadata::new(),
                children: vec![
                    ast::Node::Literal(ast::LiteralNode {
                        metadata: ast::Metadata::new(),
                        id: 2,
                        character: 'a',
                    }),
                    ast::Node::Alternation(ast::AlternationNode {
                        metadata: ast::Metadata::new(),
                        children: vec![
                            ast::Node::Literal(ast::LiteralNode {
                                metadata: ast::Metadata::new(),
                                id: 3,
                                character: 'a',
                            }),
                            ast::Node::Literal(ast::LiteralNode {
                                metadata: ast::Metadata::new(),
                                id: 4,
                                character: 'b',
                            }),
                        ],
                    }),
                ],
            }),
        ],
    });

    let mut tree = ast::Tree { root };

    tree.analyze();

    println!("{}", tree.debug_graph());
}