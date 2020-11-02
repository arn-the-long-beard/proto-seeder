# proto-seeder
A prototype to integrate to seeder later


#Description

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


After reading the file, the cli should :

- Complain if Routes not found ( then it will do nothing ) -> Done
- Complain if Model not found ( then it will do nothing ) -> Done
- Extract the Url payload from a route ( id_param, query and children) or Extract nested route if any as single variant tuple -> Done
- Extract the routes and display stats ( how many view, files it will create ) -> Done
- Create the right sub directory(ies?) with the correct name(s) ->   `#[modules_path]` -> Done
- Create the right file when route is module and create its view & init with the right payload extracted in step 2 -> Done
- Create the local view with the call to the proper model/prop if route is not module ->   `#[view = "Model/prop => local_view"]` -> Done
- Create the local guard with the call to the proper model/prop ->  ` #[guard = "Model/prop => guard  => callback_view"]` -> Done
- Add build command to test that the generated code can compile
- Add module in root file where the initial Routes is parsed

For later
- Detect if future file already exist -> Done
- If future file already exist, try to apply the command line recursively to its Routes enum?
