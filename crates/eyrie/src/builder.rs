#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::extract::Path;
    use axum::extract::State;
    use axum::routing::delete;
    use axum::routing::get;
    use axum::Router;
    use tokio::sync::RwLock;

    use super::*;
    use crate::router;

    struct DbConnection {}

    struct BookService {
        db: DbConnection,
    }

    impl BookService {
        async fn get_book(&self, id: u64) {
            todo!();
        }

        async fn delete_book(&self, id: u64) {
            todo!();
        }
    }

    // #[router("/books")]
    struct BookController {
        // #[inject]
        book_service: Arc<BookService>,
    }

    impl BookController {
        // 아래와 같은 attribute macro를 만들 예정.
        // 첫번째 인자는 반드시 아래와 같은 State(ctx): State<Arc<Self>> 타입의 인자여야 함. 인자명이 ctx일 필요는 없음.
        // 두번째 인자부터는 method_router로의 extractor들임.
        //
        // #[get("/:id")]
        async fn get_book(&self, Path(id): Path<u64>) {
            self.book_service.get_book(id).await
        }

        // 아래와 같은 attribute macro를 만들 예정.
        // 첫번째 인자는 반드시 아래와 같은 State(ctx): State<Arc<Self>> 타입의 인자여야 함. 인자명이 ctx일 필요는 없음.
        // 두번째 인자부터는 method_router로의 extractor들임.
        //

        // #[delete("/:id")]
        async fn delete_book(State(ctx): State<Arc<Self>>, Path(id): Path<u64>) {
            ctx.book_service.delete_book(id).await
        }
    }

    // 아래와 같은 코드를 macro를 통해 생성할 예정.
    // 이 때, DI container로부터 생성 및 참조해야 함.
    // let path = "/books"; // macro attr에서 가져옴.
    // let injectable = // DI container로부터 가져오거나 추가.
    // 아래 route의 경로는 macro에서 경로 문법 검증 및 붙여넣기 함.
    // macro 이름 get, delete에 따라 맞는 axum::routing::<method>를 생성함.
    // let router = Router::new()
    //     .route("/books/:id", get(BookController::get_book))
    //     .route("/books/:id", delete(BookController::delete_book))
    //     .with_state(injectable)
    //
    // 최종적으로 app이라는 router에 merge되게 해야 함.

    struct RootController {
        book_service: Arc<BookService>,
    }

    #[tokio::test]
    async fn test_router() {
        let db = DbConnection {};
        let book_service = Arc::new(BookService { db });
        let book_controller = BookController {
            book_service: book_service.clone(),
        };
        // let a = book_controller.router();
        let root_controller = RootController {
            book_service: book_service.clone(),
        };

        let app = Router::new().route("/", get(|| async { "Hello, World!" }));
        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
        todo!();
    }
}
