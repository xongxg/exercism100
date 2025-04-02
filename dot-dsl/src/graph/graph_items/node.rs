use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: String,
    pub attrs: Vec<(String, String)>,
}

impl Node {
    pub fn new(id: &str) -> Self {
        Node {
            id: id.to_string(),
            attrs: vec![],
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = attrs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        self
    }

    pub fn attr(&self, key: &str) -> Option<&str> {
        self.attrs.iter().find(|(k, _)| k == key).map(|(_, v)| &**v)
    }
}
