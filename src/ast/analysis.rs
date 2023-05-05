use super::{Node, Tree};

impl Tree {
    pub fn analyze(&mut self) {
        self.iter_pre_mut(pre_pass);
        self.iter_post_mut(post_pass);
    }
}

fn pre_pass(node: &mut Node) {
    match node {
        Node::Alternation(node) => {
            node.metadata.empty = node.children.iter().any(|child| child.metadata().empty);

            for c in node.children.iter() {
                node.metadata.first.extend(c.metadata().first.iter());
                node.metadata.last.extend(c.metadata().last.iter());
            }

        }
        Node::Concatenation(node) => {
            node.metadata.empty = node.children.iter().all(|child| child.metadata().empty);

            for c in node.children.iter() {
                node.metadata.first.extend(c.metadata().first.iter());

                if !c.metadata().empty {
                    break
                }
            }

            for c in node.children.iter().rev() {
                node.metadata.last.extend(c.metadata().last.iter());

                if !c.metadata().empty {
                    break
                }
            }
        }
        Node::Kleene(node) => {
            node.metadata.empty = true;
            node.metadata.first.extend(node.child.metadata().first.iter());
            node.metadata.last.extend(node.child.metadata().last.iter());
        }
        Node::Literal(node) => {
            node.metadata.empty = false;
            node.metadata.first.push(node.id);
            node.metadata.last.push(node.id);
        }
    }
}

fn post_pass(node: &mut Node) {
    match node {
        Node::Alternation(node) => {
            for child in node.children.iter_mut() {
                child.metadata_mut().next.extend(node.metadata.next.iter())
            }
        }
        Node::Concatenation(node) => {
            let mut buffer = node.metadata.next.clone();

            for child in node.children.iter_mut().rev() {
                let child_metadata = child.metadata_mut();
                child_metadata.next.extend(buffer.iter());

                if !child_metadata.empty {
                    buffer.clear();
                }

                buffer.extend(child_metadata.first.iter());
            }
        }
        Node::Kleene(node) => {
            let child_metadata = node.child.metadata_mut();
            child_metadata.next.extend(child_metadata.first.iter());
            child_metadata.next.extend(node.metadata.next.iter());
        }
        Node::Literal(_) => {}
    }
}