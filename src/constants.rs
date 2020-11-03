pub const _FILE_WITH_ROUTES_AND_MODEL: &str = r###"
    
// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg,>,) -> Model {
    orders
        .subscribe(Msg::UrlChanged,)
        .subscribe(Msg::UrlRequested,)
        .subscribe(Msg::UserLogged,);

    let mut router: Router<Routes,> = Router::new();
    router.init_url_and_navigation(url,);

    Model {
        theme: Theme::default(),
        login: Default::default(),
        dashboard: Default::default(),
        admin: Default::default(),
        router,
        logged_user: None,
    }
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
  
    
#[derive(Debug, PartialEq, Clone, RoutingModules)]
#[modules_path = "pages"]
pub enum Routes {
    Login {
        query: IndexMap<String, String,>, // -> http://localhost:8000/login?name=JohnDoe
    },
    #[guard = " => guard => forbidden"]
    Settings,
    #[guard = " => guard => forbidden"]
    Dashboard(pages::dashboard::Routes,), // -> http://localhost:8000/dashboard/*
    #[guard = "logged_user => admin_guard => forbidden_user"]
    Admin {
        // -> /admin/:id/*
        id: String,
        children: pages::admin::Routes,
    },
    #[default_route]
    #[view = " => not_found"] // -> http://localhost:8000/not_found*
    NotFound,
    #[view = "logged_user => forbidden"] // -> http://localhost:8000/forbidden*
    Forbidden,
    #[as_path = ""]
    #[view = "theme => home"] // -> http://localhost:8000/
    Home,
}
 "###;

pub const _FILE_WITHOUT_ROUTES_NOR_MODEL: &str = r###"
    
// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg,>,) -> Model {
    orders
        .subscribe(Msg::UrlChanged,)
        .subscribe(Msg::UrlRequested,)
        .subscribe(Msg::UserLogged,);

    let mut router: Router<Routes,> = Router::new();
    router.init_url_and_navigation(url,);

    Model {
        theme: Theme::default(),
        login: Default::default(),
        dashboard: Default::default(),
        admin: Default::default(),
        router,
        logged_user: None,
    }
}
 "###;
