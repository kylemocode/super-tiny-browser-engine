use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Node {
  children: Vec<Node>,
  node_type: NodeType,
}

#[derive(Debug)]
enum NodeType {
  Text(String),
  Element(ElementData),
  Comment(String),
}

#[derive(Debug)]
struct ElementData {
  tag_name: String,
  attributes: AttrMap,
}

impl ElementData {
  fn new(tag_name: String, attributes: AttrMap) -> ElementData {
    ElementData {
      tag_name,
      attributes,
    }
  }

  fn get_id(&self) -> Option<&String> {
    self.attributes.get("id")
  }

  fn get_classes(&self) -> HashSet<&str> {
    match self.attributes.get("class") {
      Some(s) => s.split(" ").collect(),
      None => HashSet::new(),
    }
  }
}

type AttrMap = HashMap<String, String>;

impl Node {
  fn new(node_type: NodeType, children: Vec<Node>) -> Node {
    Node {
      node_type,
      children,
    }
  }
}

fn pretty_print(n: &Node, indent_size: usize) {
  let indent = (0..indent_size).map(|_| " ").collect::<String>();

  match n.node_type {
    NodeType::Element(ref e) => println!("{}{:?}", indent, e),
    NodeType::Text(ref t) => println!("{}{}", indent, t),
    NodeType::Comment(ref c) => println!("{}<-!--{}-->", indent, c),
  }

  for child in n.children.iter() {
    pretty_print(&child, indent_size + 2);
  }

  match n.node_type {
    NodeType::Element(ref e) => println!("{}<{}/>", indent, e.tag_name),
    _ => {}
  }
}

// pub fn text(data: String) -> Node {
//   Node { children: vec![], node_type: NodeType::Text(data) }
// }

// pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
//   Node {
//       children: children,
//       node_type: NodeType::Element(ElementData {
//           tag_name: name,
//           attributes: attrs,
//       })
//   }
// }
