use yew_router::{components::RouterAnchor, prelude::Route, router::Router, Switch};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    // TODO
    // Nested content directories might not work.
    #[to = "/{page}"]
    Post(String),
    #[to = "/page-not-found"]
    PageNotFound,
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
