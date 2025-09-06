use std::collections::HashMap;

pub struct Page {
    pub name: String,
    pub params: HashMap<String, String>,
    pub render: Box<dyn Fn(&Page) -> String>,
}

impl Page {
    pub fn new<F>(name: impl Into<String>, params: HashMap<String, String>, render: F) -> Self
    where
        F: Fn(&Page) -> String + 'static,
    {
        Self {
            name: name.into(),
            params,
            render: Box::new(render),
        }
    }

    pub fn to_html(&self) -> String {
        (self.render)(self)
    }
}
