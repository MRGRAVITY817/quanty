use select::node::Node;

pub fn node_to_text(node: Node) -> String {
    node.text().trim().to_string()
}
