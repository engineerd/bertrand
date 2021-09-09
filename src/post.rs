use crate::{
    fetch::fetch_url,
    state::{AppState, State},
};
use web_sys::Node;
use yew::{prelude::*, virtual_dom::VNode};
use yewtil::future::LinkFuture;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Default, Eq, PartialEq, Properties)]
pub struct Props {
    pub page: String,
    pub state: AppState,
}

pub struct PostPage {
    initialized: bool,
    props: Props,
    content: State<String>,
    link: ComponentLink<Self>,
}

pub enum Message {
    Render,
    SetRenderState(State<String>),
}

impl Component for PostPage {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let content = State::default();
        let initialized = false;
        Self {
            initialized,
            props,
            content,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Render => {
                self.initialized = true;
                log::info!("Rendering page {}", self.props.page);

                let s = self.props.state.clone();
                let p = self.props.page.clone();
                self.link.send_future(async move {
                    let raw = fetch_url(s.info.base_url.clone() + "/content", p.clone() + ".md")
                        .await
                        .expect("cannot fetch current page");
                    match s.render(raw) {
                        Ok(s) => Message::SetRenderState(State::Success(s)),
                        Err(e) => {
                            log::error!("cannot render page: {}", e);
                            Message::SetRenderState(State::Failed)
                        }
                    }
                });
                self.link
                    .send_message(Message::SetRenderState(State::Executing));
                false
            }
            Message::SetRenderState(c) => {
                self.content = c;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        if !self.initialized {
            self.link.send_message(Message::Render);
        }

        match &self.content {
            State::NotExecuting => html! {},
            State::Executing => html! {},
            // TODO
            // If the 404 template or page are missing, this goes into a loop.
            // We should fix that.
            State::Failed => html! { <PostPage page="404" state=self.props.state.clone() /> },

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

                html! {
                    <>
                     {{ vnode }}
                    </>
                }
            }
        }
    }
}
