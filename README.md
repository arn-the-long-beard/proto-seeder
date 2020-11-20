# proto-seeder
A prototype to integrate to seeder later


# Description

This proto cli should be able to parse a rust file and read the Routes enum :

```rust

 #[derive(Debug, PartialEq, Clone, RoutingModules)]
    #[modules_path = "pages"]
     pub enum Routes {
         Other {
             id: String,
             children: Settings,
         },
         #[guard = "logged_user => admin_guard => not_authorized_view"]
         Admin { // will load module "admin.rs"
          // will load model.admin and as well
          // will check init has correct arguments
          // will check view has correct arguments
             query: IndexMap<String, String>,
         },
         #[guard = "logged_user => user_guard => not_logged_user_view"]
         Dashboard(DashboardRoutes), // will load module "dashboard"
         Profile { // will load module "profile"
             id: String,
         },
         #[guard = "logged_user => admin_guard => not_authorized_view"]
         #[view = " => my_stuff"]
         MyStuff,
         #[view = " => not_found"]
         #[default_route]
         NotFound,
         #[view = " => home"]
         #[as_path = ""]
         Root,
     }

```
# Example

See the following **lib.rs**

```rust
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
        router,
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
    #[guard = " => admin_guard => forbidden_user"]
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
router:Router<Routes>
}

pub enum Msg {
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
//---- update body ----
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
// ------ Other view content
// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
fn not_found(model: &Model) -> Node<Msg> {
    div!["not_found"]
}


```


You should get the following output from this command: 

`cargo run -- -g ./examples/backbone_app/src/lib.rs`


```
-> found 3 locals view to create
-> found 2 guards to create
-> found 3 modules to create
[+] finished parsing the file
[+] created folder ./examples/backbone_app/src/pages
[+] created file at ./examples/backbone_app/src/pages/mod.rs 
[+] updated ./examples/backbone_app/src/pages/mod.rs for import parent module => pub mod login;
pub mod dashboard;
pub mod admin;
[+] created file at ./examples/backbone_app/src/pages/login.rs 
[+] updated ./examples/backbone_app/src/pages/login.rs 
[+] updated ./examples/backbone_app/src/pages/login.rs for adding pub fn init()
[+] updated ./examples/backbone_app/src/pages/login.rs for adding pub struct Model{}
[+] updated ./examples/backbone_app/src/pages/login.rs for adding pub enum Routes{} 
[+] updated ./examples/backbone_app/src/pages/login.rs for adding pub enum Msg{}
[+] updated ./examples/backbone_app/src/pages/login.rs for adding pub fn update()
[+] updated ./examples/backbone_app/src/pages/login.rs for adding pub fn view()
[+] created file at ./examples/backbone_app/src/pages/dashboard.rs 
[+] updated ./examples/backbone_app/src/pages/dashboard.rs 
[+] updated ./examples/backbone_app/src/pages/dashboard.rs for adding pub fn init()
[+] updated ./examples/backbone_app/src/pages/dashboard.rs for adding pub struct Model{}
[+] updated ./examples/backbone_app/src/pages/dashboard.rs for adding pub enum Routes{} 
[+] updated ./examples/backbone_app/src/pages/dashboard.rs for adding pub enum Msg{}
[+] updated ./examples/backbone_app/src/pages/dashboard.rs for adding pub fn update()
[+] updated ./examples/backbone_app/src/pages/dashboard.rs for adding pub fn view()
[+] created file at ./examples/backbone_app/src/pages/admin.rs 
[+] updated ./examples/backbone_app/src/pages/admin.rs 
[+] updated ./examples/backbone_app/src/pages/admin.rs for adding pub fn init()
[+] updated ./examples/backbone_app/src/pages/admin.rs for adding pub struct Model{}
[+] updated ./examples/backbone_app/src/pages/admin.rs for adding pub enum Routes{} 
[+] updated ./examples/backbone_app/src/pages/admin.rs for adding pub enum Msg{}
[+] updated ./examples/backbone_app/src/pages/admin.rs for adding pub fn update()
[+] updated ./examples/backbone_app/src/pages/admin.rs for adding pub fn view()
[+] found file to update at ./examples/backbone_app/src/lib.rs 
[=>] No need to create view for route NotFound [ => ] as fn not_found ()
[+] found file to update at ./examples/backbone_app/src/lib.rs 
[+] updated ./examples/backbone_app/src/lib.rs for writing local view forbidden for route Forbidden
[+] updated ./examples/backbone_app/src/lib.rs for Added indentation
[+] found file to update at ./examples/backbone_app/src/lib.rs 
[=>] No need to create view for route Home [ => ] as fn home ()
[+] found file to update at ./examples/backbone_app/src/lib.rs 
[+] updated ./examples/backbone_app/src/lib.rs for writing local guard as guard
[+] updated ./examples/backbone_app/src/lib.rs for Added indentation
[=>] No need to create redirect forbidden for [ => ] guard ()
[+] found file to update at ./examples/backbone_app/src/lib.rs 
[+] updated ./examples/backbone_app/src/lib.rs for writing local guard as admin_guard
[+] updated ./examples/backbone_app/src/lib.rs for Added indentation
[+] updated ./examples/backbone_app/src/lib.rs for writing redirect for guard as forbidden_user
[+] updated ./examples/backbone_app/src/lib.rs for Added indentation
[=>] Created 4 new files
[=>] Updated 2 files
[=>] Ignored 0 files
▪▪▪▪▪ Done
```
You should have the following new code in **lib.rs** as well

```rust
fn not_found(model : &Model) -> Node<Msg>{div!["not_found"]}
fn forbidden(model : &Model) -> Node<Msg>{div!["forbidden"]}
fn guard(model : &Model) -> Option<bool> {log!("Write condition")}
fn admin_guard(model : &Model) -> Option<bool> {log!("Write condition")}
fn forbidden_user(model : &Model) -> Node<Msg>{div!["forbidden_user"]}
```

And 4 new files with TEA code inside and a new folder for this example.

# Todo 

- Complain if Routes not found ( then it will do nothing ) -> Done
- Complain if Model not found ( then it will do nothing ) -> Done
- Extract the Url payload from a route ( id_param, query and children) or Extract nested route if any as single variant tuple -> Done
- Extract the routes and display stats ( how many view, files it will create ) -> Done
- Create the right sub directory(ies?) with the correct name(s) ->   `#[modules_path]` -> Done
- Create the right file when route is module and create its view & init with the right payload extracted in step 2 -> Done
- Create the local view with the call to the proper model/prop if route is not module ->   `#[view = "Model/prop => local_view"]` -> Done
- Create the local guard with the call to the proper model/prop ->  ` #[guard = "Model/prop => guard  => callback_view"]` -> Done
- Add build command to test that the generated code can compile
- Add module in root file where the initial Routes is parsed -> Done

For later
- Detect if future file already exist -> Done
- If future file already exist, try to apply the command line recursively to its Routes enum?
- Check if content already exist, then it will not add it -> Done
- Check if local content ( local views and guard ) already exist, then it will not add it -> Done

- Check if update has been made or not and display message instead of now which is actually the number of file to update -> Done ( but need improvement )