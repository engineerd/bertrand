use crate::{
    fetch::fetch_url,
    post::PostPage,
    state::{AppState, State},
    switch::{AppRoute, AppRouter},
};
use std::collections::HashMap;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::Route;
use yewtil::future::LinkFuture;

pub enum Message {
    Initialize,
    SetRenderState(State<String>),
    SetInitializationState(State<AppState>),
    RenderCurrentPage(AppState),
}

#[derive(Clone, Default, yew::Properties)]
pub struct Properties {}

pub struct App {
    initialized: bool,
    content_cache: HashMap<String, String>,
    current_page: State<String>,
    state: State<AppState>,
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
            state: bertrand,
            link,
        }
    }

    fn update(&mut self, msg: Message) -> ShouldRender {
        match msg {
            Message::Initialize => {
                self.initialized = true;
                self.link.send_future(async move {
                    match AppState::new().await {
                        Ok(r) => Message::SetInitializationState(State::Success(r)),
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
                self.state = b;
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
        //
        // Ideally, we want to store all template assets in the local storage under
        // a revision key, and retrieve them from there.
        if !self.initialized {
            self.link.send_message(Message::Initialize);
        }

        match &self.state {
            State::NotExecuting => html! {},
            State::Executing => html! {},
            State::Failed => html! { <> <div> {{ "Internal error..." }} </div> </>},
            State::Success(c) => {
                let state = c.clone();
                html! {
                    <>
                    <main>
                    <AppRouter
                        render=AppRouter::render(move |switch: AppRoute| {

                            match switch {
                                AppRoute::Post(page) => {
                                    html! { <PostPage page=page state=state.clone() /> }
                                },
                                AppRoute::Home => html! { <PostPage page="index" state=state.clone() /> },
                                AppRoute::PageNotFound => {
                                    html! { <PostPage page="404" state=state.clone() /> }
                                }
                            }
                        })
                        redirect=AppRouter::redirect(|_route: Route| {
                            AppRoute::PageNotFound
                        })
                    />
                </main>
                    </>
                }
            }
        }
    }
}
