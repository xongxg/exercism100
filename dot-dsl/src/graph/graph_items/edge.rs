#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub attrs: Vec<(String, String)>,
}

impl Edge {
    pub fn new(from: &str, to: &str) -> Self {
        Edge {
            from: from.to_string(),
            to: to.to_string(),
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

    pub fn attr(&self, name: &str) -> Option<&str> {
        // self.attrs.iter().find(|(k, _)| k == name).map(|(_, v)| v)
        self.attrs
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| &**v)
    }
}
