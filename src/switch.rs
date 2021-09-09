use yew_router::{
    components::RouterAnchor, prelude::Route, router::Router, switch::Permissive, Switch,
};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    // TODO
    // Nested content directories might not work.
    #[to = "/{page}"]
    Post(String),
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/!"]
    Home,
}

impl AppRoute {
    pub fn into_route(self) -> Route {
        Route::from(self)
    }
}

pub type AppRouter = Router<AppRoute>;
pub type AppAnchor = RouterAnchor<AppRoute>;
