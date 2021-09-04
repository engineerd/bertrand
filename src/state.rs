use crate::{
    error::BertrandError,
    fetch::{fetch_url, get_data},
};
use handlebars::Handlebars;
use pulldown_cmark as markdown;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};
use yew::{utils, Html};

/// The symbol separating the frontmatter from content in pages.
const DOC_SEPERATOR: &str = "\n---\n";
/// The website configuration file.
const CONFIG_FILE: &str = "example/bertrand.yaml";
/// The default template to be used if none is supplied.
const DEFAULT_TEMPLATE: &str = "main";

/// Metadata about the site.
/// This is a `bertrand.yaml` file at the root of the website.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SiteInfo {
    /// The title of the website.
    pub title: String,
    /// List of scripts used to render the website.
    pub scripts: Vec<String>,
    /// List of markdown documents that will appear in an "index" page (for blogs).
    pub content: Vec<String>,
    /// List of templates used to render the website.
    pub templates: Vec<String>,
    /// The base URL of the website.
    /// Used to construct the path for scripts, content, and templates.
    pub base_url: String,

    /// Optional logo.
    pub logo: Option<String>,
    /// Optional about information.
    pub about: Option<String>,
    ///  A map of string/string pairs that are user-customizable.
    pub extra: BTreeMap<String, String>,
}

impl SiteInfo {
    pub async fn new(base_url: String, file: String) -> Result<Self, BertrandError> {
        log::info!("Fetching site info.");
        let raw_config = fetch_url(base_url, file).await?;
        log::info!("raw config: {}", raw_config);
        Ok(serde_yaml::from_str(&raw_config)?)
    }
}

/// Metadata about a page.
/// It is contained in the markdown file of the page,
/// separated by `DOC_SEPARATOR` (`\n---\n`).
#[derive(Clone, Deserialize, Serialize)]
pub struct Frontmatter {
    /// The title of the document
    pub title: String,
    /// A short description of the document.
    pub description: Option<String>,
    /// The date of the document.
    pub date: String,
    /// The author of the document.
    pub author: String,
    /// The template to be used. If None, the `main` template is used.
    pub template: Option<String>,
    /// A map of string/string pairs that are user-customizable.
    pub extra: Option<HashMap<String, String>>,
}

/// A page containing metadata and content.
#[derive(Serialize)]
pub struct Content {
    pub frontmatter: Frontmatter,
    pub body: String,
}

impl Content {
    /// Render the body using a Markdown renderer
    pub fn render_markdown(&self) -> String {
        let mut buf = String::new();

        let opt = markdown::Options::all();
        let parser = markdown::Parser::new_ext(&self.body, opt);
        markdown::html::push_html(&mut buf, parser);

        buf
    }
}

impl FromStr for Content {
    type Err = anyhow::Error;
    fn from_str(full_document: &str) -> Result<Self, Self::Err> {
        let (yaml_text, body) = full_document
            .split_once(DOC_SEPERATOR)
            .unwrap_or(("title = 'Untitled'", full_document));
        let frontmatter: Frontmatter = serde_yaml::from_str(yaml_text)?;
        let body = Content {
            frontmatter: frontmatter.clone(),
            body: body.to_owned(),
        };
        let body = body.render_markdown();

        Ok(Content { frontmatter, body })
    }
}

#[derive(Serialize)]
pub struct TemplateContext {
    request: RequestValues,
    page: Content,
    site: SiteInfo,
}

#[derive(Serialize)]
pub struct RequestValues {}

/// The possible states a rendering request can be in.
#[derive(Clone)]
pub enum State<T> {
    NotExecuting,
    Executing,
    Success(T),
    Failed,
}

impl<T> Default for State<T> {
    fn default() -> Self {
        Self::Executing
    }
}

#[derive(Clone, Debug, Default)]
pub struct Renderer {
    pub info: SiteInfo,
    pub base_url: String,
    scripts: HashMap<String, String>,
    templates: HashMap<String, String>,
}

impl Renderer {
    /// Create a new instance of the application state.
    pub async fn new() -> Result<Renderer, BertrandError> {
        log::info!("Creating instance of Bertrand.");
        let base_url = utils::window().location().origin().unwrap();
        let info = SiteInfo::new(base_url.clone(), CONFIG_FILE.into()).await?;
        log::info!("Site info: {:?}", info);

        let scripts = get_data(
            info.base_url.clone() + "/scripts",
            info.scripts.clone(),
            Some("rhai"),
        )
        .await?;
        let templates = get_data(
            info.base_url.clone() + "/templates",
            info.templates.clone(),
            Some("hbs"),
        )
        .await?;

        Ok(Self {
            info,
            base_url,
            scripts,
            templates,
        })
    }

    /// Render a markdown page and return its HTML representation.
    pub fn render(&self, content: String) -> Result<String, BertrandError> {
        let content = Content::from_str(&content)?;
        let info = &self.info.clone();

        let tpl = content
            .frontmatter
            .template
            .clone()
            .unwrap_or_else(|| DEFAULT_TEMPLATE.to_owned());

        let ctx = TemplateContext {
            request: RequestValues {},
            page: content,
            site: info.clone(),
        };

        // This is a workaround for the lifetime parameter for the
        // handlebars renderer.
        // It means that for every rendered page we create a new instance
        // of the renderer.
        let mut renderer = Handlebars::default();
        self.load(&mut renderer)?;

        Ok(renderer.render(&tpl, &ctx)?)
    }

    /// Load the script and templates
    fn load(&self, renderer: &mut Handlebars) -> Result<(), anyhow::Error> {
        // Load all the templates.
        for (name, template) in &self.templates {
            renderer.register_template_string(name, template)?;
        }

        // Load all the helper scripts.
        for (name, script) in &self.scripts {
            renderer.register_script_helper(name, script)?;
        }

        Ok(())
    }
}