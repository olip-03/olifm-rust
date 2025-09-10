use std::collections::HashMap;

pub struct Page {
    pub name: String,
    pub params: HashMap<String, String>,
    pub render: Box<dyn Fn(&Page) -> String>,
    on_after_render: Option<Box<dyn Fn()>>,
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
            on_after_render: None,
        }
    }

    pub fn with_on_after_render(mut self, on_after_render: Option<Box<dyn Fn()>>) -> Self {
        self.on_after_render = on_after_render;
        self
    }

    pub fn to_html(&self) -> String {
        let rendered = (self.render)(self);

        // Check and trigger the on_after_render callback if it is set
        if let Some(callback) = &self.on_after_render {
            callback(); // Correctly invoke the callback function
        }

        rendered
    }
}
