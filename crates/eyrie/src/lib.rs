mod module;
mod router;

use proc_macro::TokenStream;

use crate::{module::module_impl, router::router_impl};

#[proc_macro_attribute]
pub fn module(attr: TokenStream, item: TokenStream) -> TokenStream {
    module_impl(attr, item)
}

#[proc_macro_attribute]
pub fn router(attr: TokenStream, item: TokenStream) -> TokenStream {
    router_impl(attr, item)
}

// #[router("/users")]
// struct UsersRouter {
//     #[inject]
//     users_service: UsersService,
// }

// impl UsersRouter {
//     #[get("/")]
//     async fn get_users(&self, ...) {
//         self.users_service.get_users(...)
//     }

//     #[post("/")]
//     async fn create_user(&self, ...) {
//         self.users_service.create_user(...)
//     }

//     #[delete("/:id")]
//     async fn delete_user(Path(id): Path<u64>) {
//         self.users_service.delete_user(id)
//     }
// }

// #[injectable]
// struct UsersService;

// impl UsersService {
//     async fn get_users(&self, ...) {
//         todo!();
//     }

//     async fn create_user(&self, ...) {
//         todo!();
//     }

//     async fn delete_user(&self, ...) {
//         todo!();
//     }
// }

// // #[module]
// // struct UsersModule;

// // #[module]
// // struct AppModule;

// // src/
// //     app/mod.rs
// //     users/
// //         mod.rs
// //         router.rs
// //         service.rs

// struct Module;

// struct ModuleBuilder;

// impl ModuleBuilder {
//     fn import(&mut self) -> &mut Self {
//         self
//     }

//     fn export(&mut self) -> &mut Self {
//         self
//     }

//     fn build(self) -> Module {
//         todo!();
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use axum::{routing::get, Router};

//     struct AppModule {
//         users_module: UsersModule,
//     }

//     struct UsersModule {
//         users_router: UsersRouter,
//         users_service: UsersService,
//     }

//     struct UsersRouter {
//         users_service: &mut UsersService,
//     }

//     struct UsersService {
//         db: Db,
//     }

//     struct Db;

//     #[tokio::test]
//     async fn test_a() {
//         let app = Router::new().route("/", get(|| async { "Hello, World!" }));

//         // run our app with hyper, listening globally on port 3000
//         let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//         axum::serve(listener, app).await.unwrap();
//     }
// }
