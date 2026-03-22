use actix_web::{HttpRequest, HttpResponse};
use auth_kernel::user_session::transactions::get::GetUserSession;
use glue::{
    errors::{NanoServiceError, NanoServiceErrorStatus},
    token::HeaderToken,
};
use to_do_core::{
    api::basic_actions::{delete::delete as delete_core, get::get_all as get_all_core},
    structs::ToDoItem,
};
use to_do_dal::to_do_items::transactions::{delete::DeleteOne, get::GetAll};

pub async fn delete_by_name<T: DeleteOne + GetAll, X: GetUserSession>(
    token: HeaderToken,
    req: HttpRequest,
) -> Result<HttpResponse, NanoServiceError> {
    let session = X::get_user_session(token.unique_id).await?;
    match req.match_info().get("name") {
        Some(name) => {
            delete_core::<T>(name, session.user_id).await?;
        }
        None => {
            return Err(NanoServiceError::new(
                "Name not provided".to_string(),
                NanoServiceErrorStatus::BadRequest,
            ));
        }
    };
    Ok(HttpResponse::Ok().json(get_all_core::<T>(session.user_id).await?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        self,
        body::MessageBody,
        dev::ServiceResponse,
        http::{header::ContentType, ConnectionType},
        test::{call_service, init_service, TestRequest},
        web, App,
    };

    use actix_http::Request;
    use auth_kernel::user_session::schema::UserSession;
    use glue::token::HeaderToken;
    use std::future::Future;
    use to_do_dal::to_do_items::schema::{AllToDoItems, ToDoItem};
    use to_do_dal::to_do_items::transactions::delete::DeleteOne;

    fn generate_to_do_item() -> ToDoItem {
        ToDoItem {
            id: 1,
            title: "test".to_string(),
            status: "PENDING".to_string(),
        }
    }

    fn generate_get_all_return() -> Vec<ToDoItem> {
        vec![generate_to_do_item()]
    }

    // Mock Database Handle
    struct MockDbHandle;

    type DeleteOneResponse = Result<ToDoItem, NanoServiceError>;

    impl DeleteOne for MockDbHandle {
        fn delete_one(
            title: String,
            user_id: i32,
        ) -> impl Future<Output = DeleteOneResponse> + Send {
            async fn run(title: String) -> DeleteOneResponse {
                if title == "coding" {
                    return Ok(generate_to_do_item());
                }

                Err(NanoServiceError {
                    message: "Item not found".to_string(),
                    status: NanoServiceErrorStatus::NotFound,
                })
            }
            run(title)
        }
    }

    impl GetAll for MockDbHandle {
        fn get_all(
            user_id: i32,
        ) -> impl Future<Output = Result<Vec<ToDoItem>, NanoServiceError>> + Send {
            async fn run(user_id: i32) -> Result<Vec<ToDoItem>, NanoServiceError> {
                if user_id == 2 {
                    return Err(NanoServiceError::new(
                        "error getting items got get all".to_string(),
                        NanoServiceErrorStatus::Unknown,
                    ));
                }
                Ok(generate_get_all_return())
            }
            run(user_id)
        }
    }

    // Mock User Cache Handle
    struct MockUserSessionHandle;

    impl GetUserSession for MockUserSessionHandle {
        fn get_user_session(
            unique_id: String,
        ) -> impl Future<Output = Result<UserSession, NanoServiceError>> {
            async fn run(unique_id: String) -> Result<UserSession, NanoServiceError> {
                if unique_id == "break" {
                    return Err(NanoServiceError::new(
                        "User not found".to_string(),
                        NanoServiceErrorStatus::NotFound,
                    ));
                }
                if unique_id == "2" {
                    return Ok(UserSession { user_id: 2 });
                }
                Ok(UserSession { user_id: 1 })
            }
            run(unique_id)
        }
    }

    // Mock Service
    async fn run_request(req: Request) -> ServiceResponse {
        let service = delete_by_name::<MockDbHandle, MockUserSessionHandle>;
        let app = init_service(App::new().route("/delete/{name}", web::delete().to(service))).await;
        call_service(&app, req).await
    }

    #[tokio::test]
    async fn test_delete_ok() {
        std::env::set_var("JWT_SECRET", "secret");

        let req = TestRequest::delete()
            .insert_header(ContentType::json())
            .insert_header((
                "token",
                HeaderToken {
                    unique_id: "test_id".to_string(),
                }
                .encode()
                .unwrap(),
            ))
            .uri("/delete/coding")
            .to_request();
        let resp = run_request(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();
        let body: AllToDoItems = serde_json::from_str(body_str).unwrap();

        assert_eq!(status, 200);
        assert_eq!(
            body,
            AllToDoItems::from_vec(generate_get_all_return()).unwrap()
        );
    }

    #[tokio::test]
    async fn test_delete_invalid_token() {
        // configure the service
        std::env::set_var("JWT_SECRET", "secret");

        // make the request and get the response
        let req = TestRequest::delete()
            .insert_header(ContentType::json())
            .insert_header(("token", "test"))
            .uri("/delete/coding")
            .to_request();
        let resp = run_request(req).await;

        // extract the status and body
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        // assert the status and body
        assert_eq!(status, 401);
        assert_eq!(body_str, "\"token not a valid string\"");
    }

    #[tokio::test]
    async fn test_delete_user_not_found() {
        // configure the service
        std::env::set_var("JWT_SECRET", "secret");

        // make the request and get the response
        let req = TestRequest::delete()
            .insert_header(ContentType::json())
            .insert_header((
                "token",
                HeaderToken {
                    unique_id: "break".to_string(),
                }
                .encode()
                .unwrap(),
            ))
            .uri("/delete/coding")
            .to_request();

        let resp = run_request(req).await;

        // extract the status and body
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        // assert the status and body
        assert_eq!(status, 404);
        assert_eq!(body_str, "\"User not found\"");
    }

    #[tokio::test]
    async fn test_delete_item_not_found() {
        // configure the service
        std::env::set_var("JWT_SECRET", "secret");

        // make the request and get the response
        let req = TestRequest::delete()
            .insert_header(ContentType::json())
            .insert_header((
                "token",
                HeaderToken {
                    unique_id: "test_id".to_string(),
                }
                .encode()
                .unwrap(),
            ))
            .uri("/delete/break")
            .to_request();
        let resp = run_request(req).await;

        // extract the status and body
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        // assert the status and body
        assert_eq!(status, 404);
        assert_eq!(body_str, "\"Item not found\"");
    }

    #[tokio::test]
    async fn test_delete_get_all_error() {
        // configure the service
        std::env::set_var("JWT_SECRET", "secret");

        // make the request and get the response
        let req = TestRequest::delete()
            .insert_header(ContentType::json())
            .insert_header((
                "token",
                HeaderToken {
                    unique_id: "2".to_string(),
                }
                .encode()
                .unwrap(),
            ))
            .uri("/delete/coding")
            .to_request();
        let resp = run_request(req).await;

        // extract the status and body
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        // assert the status and body
        assert_eq!(status, 500);
        assert_eq!(body_str, "\"error getting items got get all\"");
    }
}
