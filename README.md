# proto-seeder
A Cli prototype to integrate to seeder later.
It generates files and methods for a Seed app from a `Route` `enum`.


# Description

This proto cli should be able to parse a rust file and read the Route enum :

```rust

 #[derive(Debug, PartialEq, Clone, RoutingModules)]
    #[modules_path = "pages"]
     pub enum Route {
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

You should get the following output from this command at root of your project: 

`proto_seeder ./src/lib.rs`

if you go to `/src`  and run : 

`proto_seeder lib.rs`

Will not work because of https://github.com/arn-the-long-beard/proto-seeder/issues/1

Here is an example of output with the example.

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

- [x] Complain if Routes not found ( then it will do nothing ) 
- [x] Complain if Model not found ( then it will do nothing ) 
- [x] Extract the Url payload from a route ( id_param, query and children) or Extract nested route if any as single variant tuple 
- [x] Extract the routes and display stats ( how many view, files it will create ) 
- [x] Create the right sub directory(ies?) with the correct name(s) ->   `#[modules_path]`
- [x] Create the right file when route is module and create its view & init with the right payload extracted in step 2 -> Done
- [x] Create the local view with the call to the proper model/prop if route is not module ->   `#[view = "Model/prop => local_view"]` 
- [x] Create the local guard with the call to the proper model/prop ->  ` #[guard = "Model/prop => guard  => callback_view"]` 
- [ ] Add build command to test that the generated code can compile
- [x] Add module in root file where the initial Routes is parsed 

For later
- [x] Detect if future file already exist 
- [x] If future file already exist, try to apply the command line recursively to its Routes enum?
- [x] Check if content already exist, then it will not add it 
- [x] Check if local content ( local views and guard ) already exist, then it will not add it 

- [x] Check if update has been made or not and display message instead of now which is actually the number of file to update. Needs improvement.
- [ ] Generate implementation of the router in lib.rs