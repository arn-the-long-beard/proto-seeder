mod request;
use seed::{prelude::*, *};
extern crate heck;
use crate::{
    models::user::{LoggedData, Role},
    theme::Theme,
    top_bar::TopBar,
};
#[macro_use]
extern crate seed_routing;
use seed_routing::{View, *};
pub mod models;

mod theme;
mod top_bar;

use std::fmt::Debug;

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .subscribe(Msg::UrlRequested)
        .subscribe(Msg::UserLogged);

    let mut router: Router<Routes> = Router::new();
    router.init_url_and_navigation(url);

    Model {
        theme: Theme::default(),
        login: Default::default(),
        dashboard: Default::default(),
        admin: Default::default(),
        router,
        logged_user: None,
    }
}
#[derive(Debug, PartialEq, Clone, RoutingModules)]
#[modules_path = "pages"]
pub enum Routes {
    #[guard = " => guard => forbidden"]
    Login {
        query: IndexMap<String, String>, // -> http://localhost:8000/login?name=JohnDoe
    },
    #[guard = " => guard => forbidden"]
    Dashboard(pages::dashboard::Routes), // -> http://localhost:8000/dashboard/*
    #[guard = " logged_user => admin_guard => forbidden_user"]
    Admin {
        // -> /admin/:id/*
        id: String,
        children: pages::admin::Routes,
    },
    #[default_route]
    #[view = " => not_found"] // -> http://localhost:8000/not_found*
    NotFound,
    #[view = " => forbidden"] // -> http://localhost:8000/forbidden*
    Forbidden,
    #[as_path = ""]
    #[view = "theme => home"] // -> http://localhost:8000/
    Home,
}

// ------ ------
//     Model
// ------ ------

struct Model {
    pub login: pages::login::Model,
    pub dashboard: pages::dashboard::Model,
    pub admin: pages::admin::Model,
    router: Router<Routes>,
    logged_user: Option<LoggedData>,
    theme: Theme,
}

// ------ ------
//    Update
// ------ ------
/// Root actions for your app.
/// Each component will have single action/message mapped to its message later
/// in update

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    UrlRequested(subs::UrlRequested),
    Login(pages::login::Msg),
    Admin(pages::admin::Msg),
    UserLogged(LoggedData),
    Dashboard(pages::dashboard::Msg),
    GoBack,
    GoForward,
    Logout,
    GoLogin,
    SwitchToTheme(Theme),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.router.confirm_navigation(url);
            if let Some(current_route) = model.router.current_route.clone() {
                current_route.init(model, orders);
            }
        }
        Msg::UrlRequested(_) => {}
        Msg::Login(login_message) => pages::login::update(
            login_message,
            &mut model.login,
            &mut orders.proxy(Msg::Login),
        ),
        Msg::Dashboard(dashboard_message) => pages::dashboard::update(
            dashboard_message,
            &mut model.dashboard,
            &mut orders.proxy(Msg::Dashboard),
        ),

        Msg::Admin(admin_msg) => {
            pages::admin::update(admin_msg, &mut model.admin, &mut orders.proxy(Msg::Admin))
        }
        Msg::UserLogged(user) => {
            model.logged_user = Some(user);
        }

        Msg::SwitchToTheme(theme) => model.theme = theme,

        Msg::GoBack => {
            model
                .router
                .request_moving_back(|r| orders.notify(subs::UrlRequested::new(r)));
        }
        Msg::GoForward => {
            model
                .router
                .request_moving_forward(|r| orders.notify(subs::UrlRequested::new(r)));
        }
        Msg::Logout => model.logged_user = None,
        Msg::GoLogin => {
            model.router.current_route = Some(Routes::Login {
                query: IndexMap::new(),
            })
        }
    }
}

// ------ ------
//     View
// ------ ------
/// View function which renders stuff to html
fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![
        header(&model),
        if let Some(route) = &model.router.current_route {
            route.view(model)
        } else {
            home(&model.theme)
        },
    ]
}

fn header(model: &Model) -> Node<Msg> {
    div![
        TopBar::new(who_is_connected(model))
            .style(model.theme.clone())
            .set_user_login_state(model.logged_user.is_some())
            .content(div![
                style! {St::Display => "flex" },
                button![
                    "back",
                    attrs! {
                        At::Disabled  =>   (!model.router.can_back()).as_at_value(),
                    },
                    ev(Ev::Click, |_| Msg::GoBack)
                ],
                button![
                    "forward",
                    attrs! {
                        At::Disabled =>  (!model.router.can_forward()).as_at_value(),
                    },
                    ev(Ev::Click, |_| Msg::GoForward)
                ],
                span![style! {St::Flex => "5" },],
                build_account_button(model.logged_user.is_some())
            ]),
        render_route(model)
    ]
}

fn who_is_connected(logged_user: &Model) -> String {
    if let Some(user) = logged_user {
        let full_welcome = format!("Welcome {} {}", user.first_name, user.last_name);
        full_welcome
    } else {
        "Welcome Guest".to_string()
    }
}

fn build_account_button(user_logged_in: bool) -> Node<Msg> {
    if user_logged_in {
        span![button![
            "logout ",
            ev(Ev::Click, |_| Msg::Logout),
            C!["user_button"],
            i![C!["far fa-user-circle"]]
        ]]
    } else {
        span![button![
            "sign in ",
            ev(Ev::Click, |_| Msg::GoLogin),
            C!["user_button"],
            i![C!["fas fa-user-circle"]]
        ]]
    }
}

fn make_query_for_john_doe() -> IndexMap<String, String> {
    let mut query: IndexMap<String, String> = IndexMap::new();
    query.insert("name".to_string(), "JohnDoe".to_string());
    query
}

fn render_route(model: &Model) -> Node<Msg> {
    ul![
        generate_root_nodes(&model.router),
        li![a![C!["route"], "Admin",]],
        ul![generate_admin_nodes(&model, &model.router)],
        li![a![C!["route"], "Dashboard",]],
        ul![generate_dashboard_nodes(&model, &model.router)],
    ]
}

fn generate_root_routes() -> Vec<(Routes, &'static str)> {
    let mut vec: Vec<(Routes, &'static str)> = vec![];
    vec.push((
        Routes::Login {
            query: IndexMap::new(),
        },
        "Login",
    ));
    vec.push((
        Routes::Login {
            query: make_query_for_john_doe(),
        },
        "Login for JohnDoe",
    ));
    vec.push((Routes::NotFound, "NotFound"));
    vec.push((Routes::Home, "Home"));
    vec
}

fn generate_root_nodes(router: &Router<Routes>) -> Vec<Node<Msg>> {
    let mut list: Vec<Node<Msg>> = vec![];
    for route in generate_root_routes().iter() {
        list.push(li![a![
            C![
                "route",
                IF!( router.is_current_route(&route.0 ) => "active-route" )
            ],
            attrs! { At::Href => &route.0.to_url() },
            route.1,
        ]])
    }
    list
}

fn generate_admin_routes() -> Vec<(Routes, &'static str)> {
    let mut vec: Vec<(Routes, &'static str)> = vec![];
    vec.push((
        Routes::Admin {
            id: "1".to_string(),
            children: pages::admin::Routes::Root,
        },
        "Admin Project 1",
    ));
    vec.push((
        Routes::Admin {
            id: "2".to_string(),
            children: pages::admin::Routes::Root,
        },
        "Admin Project 2",
    ));
    vec.push((
        Routes::Admin {
            id: "3".to_string(),
            children: pages::admin::Routes::Root,
        },
        "Admin Project 3",
    ));
    vec.push((
        Routes::Admin {
            id: "3".to_string(),
            children: pages::admin::Routes::NotFound,
        },
        "Not found project 3",
    ));
    vec.push((
        Routes::Admin {
            id: "1".to_string(),
            children: pages::admin::Routes::Manager,
        },
        "Manage project 1",
    ));
    vec
}

fn generate_admin_nodes(model: &Model, router: &Router<Routes>) -> Vec<Node<Msg>> {
    let mut list: Vec<Node<Msg>> = vec![];
    for route in generate_admin_routes().iter() {
        list.push(   li![a![
            C![
                "route",
                IF!( router.is_current_route(&route.0 ) => "active-route" )
                           IF!(admin_guard(model).is_none() => "locked-route"   ),
                    IF!(admin_guard(model).is_some() && !admin_guard(model).unwrap() => "locked-admin-route" )
            ],
            attrs! { At::Href => &route.0.to_url() },
           route.1,
        ]])
    }
    list
}

fn generate_dashboard_routes() -> Vec<(Routes, &'static str)> {
    let mut vec: Vec<(Routes, &'static str)> = vec![];
    vec.push((Routes::Dashboard(pages::dashboard::Routes::Root), "Profile"));
    vec.push((
        Routes::Dashboard(pages::dashboard::Routes::Message),
        "Message",
    ));
    vec.push((
        Routes::Dashboard(pages::dashboard::Routes::Statistics),
        "Statistics",
    ));
    vec.push((
        Routes::Dashboard(pages::dashboard::Routes::Tasks {
            query: IndexMap::new(),
            children: pages::dashboard::tasks::Routes::Root,
        }),
        "Tasks",
    ));
    vec.push((
        Routes::Dashboard(pages::dashboard::Routes::Tasks {
            query: make_query(),
            children: pages::dashboard::tasks::Routes::Root,
        }),
        "Tasks with url query",
    ));
    vec
}

fn generate_dashboard_nodes(model: &Model, router: &Router<Routes>) -> Vec<Node<Msg>> {
    let mut list: Vec<Node<Msg>> = vec![];
    for route in generate_dashboard_routes().iter() {
        list.push(li![a![
            C![
                "route",
                IF!( router.is_current_route(&route.0 ) => "active-route" )
                           IF!(guard(model).is_none() => "locked-route"   ),
            ],
            attrs! { At::Href => &route.0.to_url() },
            route.1,
        ]])
    }
    list
}

fn make_query() -> IndexMap<String, String> {
    let mut index_map: IndexMap<String, String> = IndexMap::new();
    index_map.insert("select1".to_string(), "1".to_string());
    index_map
}

fn home(theme: &Theme) -> Node<Msg> {
    div![
        div!["Welcome home!"],
        match theme {
            Theme::Dark => {
                button![
                    "Switch to Light",
                    ev(Ev::Click, |_| Msg::SwitchToTheme(Theme::Light))
                ]
            }
            Theme::Light => {
                button![
                    "Switch to Dark",
                    ev(Ev::Click, |_| Msg::SwitchToTheme(Theme::Dark))
                ]
            }
        }
    ]
}
// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
mod pages;fn not_found(model : &Model) -> Node<Msg>{div!["not_found"]}
fn forbidden(model : &Model) -> Node<Msg>{div!["forbidden"]}

pub fn guard(model : &Model) -> Option<bool>
{
if 1+1 ==2 {
 log!("Need to implement nice conditions");
}

None
}


pub fn admin_guard(logged_user : &Option<LoggedData>) -> Option<bool>
{
if 1+1 ==2 {
 log!("Need to implement nice conditions");
}

None
}

fn forbidden_user(logged_user : &Option<LoggedData>) -> Node<Msg>{div!["forbidden_user"]}
