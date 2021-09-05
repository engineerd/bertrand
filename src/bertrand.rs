use crate::{
    fetch::fetch_url,
    state::{Renderer, State},
};
use std::collections::HashMap;
use web_sys::Node;
use yew::{html, utils, virtual_dom::VNode, Component, ComponentLink, Html, ShouldRender};
use yewtil::future::LinkFuture;

pub enum Message {
    Initialize,
    SetRenderState(State<String>),
    SetInitializationState(State<Renderer>),
    RenderCurrentPage(Renderer),
}

#[derive(Clone, Default, yew::Properties)]
pub struct Properties {}

pub struct App {
    initialized: bool,
    content_cache: HashMap<String, String>,
    current_page: State<String>,
    renderer: State<Renderer>,
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = Message;
    type Properties = Properties;

    fn create(_props: Properties, link: ComponentLink<Self>) -> Self {
        log::info!("Creating main component.");

        let initialized = false;
        let current_page = State::default();
        let content_cache = HashMap::new();
        let bertrand = State::default();

        Self {
            initialized,
            content_cache,
            current_page,
            renderer: bertrand,
            link,
        }
    }

    fn update(&mut self, msg: Message) -> ShouldRender {
        match msg {
            Message::Initialize => {
                self.initialized = true;
                self.link.send_future(async move {
                    match Renderer::new().await {
                        Ok(r) => Message::RenderCurrentPage(r),
                        Err(e) => {
                            log::error!("Error: {}", e);
                            Message::SetRenderState(State::Failed)
                        }
                    }
                });
                self.link
                    .send_message(Message::SetRenderState(State::Executing));
                false
            }

            Message::SetRenderState(c) => {
                self.current_page = c;
                true
            }
            Message::SetInitializationState(b) => {
                self.renderer = b;
                true
            }
            Message::RenderCurrentPage(r) => {
                let current_page = "blog/what-is-markdown".to_string();

                log::info!("Rendering page {}", current_page);

                match self.content_cache.get(&current_page.clone()) {
                    Some(c) => self
                        .link
                        .send_message(Message::SetRenderState(State::Success(c.clone()))),
                    None => {
                        log::info!("Fetching page contents.");
                        let r = r.clone();

                        self.link.send_future(async move {
                            let page = fetch_url(
                                r.info.base_url.clone() + "/content",
                                current_page.clone() + ".md",
                            )
                            .await
                            .expect("cannot fetch current page");
                            let c = r.render(page).expect("cannot render page");
                            Message::SetRenderState(State::Success(c))
                        });
                    }
                };

                self.link
                    .send_message(Message::SetRenderState(State::Executing));
                false
            }
        }
    }

    fn change(&mut self, _props: Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // If this is the first page load, initialize the application state.
        // This is a costly operation, as it downloads all scripts and templates,
        // and we want to make sure it is only done once, when the app first loads.
        if !self.initialized {
            self.link.send_message(Message::Initialize);
        }

        match &self.current_page {
            State::NotExecuting => html! { "Unknown error" },
            State::Executing => html! { "Working..." },
            State::Success(c) => {
                let div = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element("html")
                    .unwrap();

                div.set_inner_html(c);
                div.set_attribute("id", "bertrand").unwrap();

                let node = Node::from(div);
                let vnode = VNode::VRef(node);
                vnode
            }
            State::Failed => html! { "Error" },
        }
    }
}

fn current_page() -> String {
    utils::window().location().pathname().unwrap()
}
