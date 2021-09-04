use std::fmt::{self, Debug, Display, Formatter};

use anyhow::Error;

#[derive(Debug)]
pub enum BertrandError {
    RenderError(handlebars::RenderError),
    YamlError(serde_yaml::Error),
    TemplateError(handlebars::TemplateError),
    JsError(wasm_bindgen::JsValue),
    Other(Error),
}

impl Display for BertrandError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl From<Error> for BertrandError {
    fn from(e: Error) -> Self {
        Self::Other(e)
    }
}

impl From<serde_yaml::Error> for BertrandError {
    fn from(e: serde_yaml::Error) -> Self {
        Self::YamlError(e)
    }
}

impl From<handlebars::TemplateError> for BertrandError {
    fn from(e: handlebars::TemplateError) -> Self {
        Self::TemplateError(e)
    }
}

impl From<handlebars::RenderError> for BertrandError {
    fn from(e: handlebars::RenderError) -> Self {
        Self::RenderError(e)
    }
}

impl From<wasm_bindgen::JsValue> for BertrandError {
    fn from(e: wasm_bindgen::JsValue) -> Self {
        Self::JsError(e)
    }
}
