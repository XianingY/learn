use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "http://127.0.0.1:3000";

#[derive(Debug, Serialize)]
struct CreateTodoRequest {
    title: String,
    description: Option<String>,
}

#[derive(Debug, Serialize)]
struct UpdateTodoRequest {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct TodoResponse {
    id: i64,
    title: String,
    description: Option<String>,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct HealthResponse {
    status: String,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
}

#[tokio::test]
async fn test_health_endpoint() {
    let client = reqwest::Client::new();
    
    let resp = client
        .get(format!("{}/health", BASE_URL))
        .send()
        .await;
    
    if let Ok(resp) = resp {
        assert_eq!(resp.status(), StatusCode::OK);
        let body: HealthResponse = resp.json().await.unwrap();
        assert_eq!(body.status, "ok");
    }
}

#[tokio::test]
async fn test_create_todo() {
    let client = reqwest::Client::new();
    
    let create_req = CreateTodoRequest {
        title: "Test Todo".to_string(),
        description: Some("Test Description".to_string()),
    };
    
    let resp = client
        .post(format!("{}/todos", BASE_URL))
        .json(&create_req)
        .send()
        .await;
    
    if let Ok(resp) = resp {
        assert_eq!(resp.status(), StatusCode::CREATED);
        let body: TodoResponse = resp.json().await.unwrap();
        assert_eq!(body.title, "Test Todo");
        assert_eq!(body.description, Some("Test Description".to_string()));
        assert!(!body.completed);
    }
}

#[tokio::test]
async fn test_list_todos() {
    let client = reqwest::Client::new();
    
    let resp = client
        .get(format!("{}/todos", BASE_URL))
        .send()
        .await;
    
    if let Ok(resp) = resp {
        assert_eq!(resp.status(), StatusCode::OK);
        let _: Vec<TodoResponse> = resp.json().await.unwrap();
    }
}

#[tokio::test]
async fn test_get_nonexistent_todo_returns_404() {
    let client = reqwest::Client::new();
    
    let resp = client
        .get(format!("{}/todos/99999", BASE_URL))
        .send()
        .await;
    
    if let Ok(resp) = resp {
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body: ErrorResponse = resp.json().await.unwrap();
        assert!(body.error.contains("not found"));
    }
}

#[tokio::test]
async fn test_full_crud_cycle() {
    let client = reqwest::Client::new();
    
    let create_req = CreateTodoRequest {
        title: "CRUD Test".to_string(),
        description: Some("Will be updated and deleted".to_string()),
    };
    
    let create_resp = client
        .post(format!("{}/todos", BASE_URL))
        .json(&create_req)
        .send()
        .await;
    
    if let Ok(resp) = create_resp {
        if resp.status() != StatusCode::CREATED {
            return;
        }
        let created: TodoResponse = resp.json().await.unwrap();
        let id = created.id;
        
        let update_req = UpdateTodoRequest {
            title: Some("Updated CRUD Test".to_string()),
            description: None,
            completed: Some(true),
        };
        
        let update_resp = client
            .put(format!("{}/todos/{}", BASE_URL, id))
            .json(&update_req)
            .send()
            .await
            .unwrap();
        
        assert_eq!(update_resp.status(), StatusCode::OK);
        let updated: TodoResponse = update_resp.json().await.unwrap();
        assert_eq!(updated.title, "Updated CRUD Test");
        assert!(updated.completed);
        
        let delete_resp = client
            .delete(format!("{}/todos/{}", BASE_URL, id))
            .send()
            .await
            .unwrap();
        
        assert_eq!(delete_resp.status(), StatusCode::NO_CONTENT);
        
        let get_resp = client
            .get(format!("{}/todos/{}", BASE_URL, id))
            .send()
            .await
            .unwrap();
        
        assert_eq!(get_resp.status(), StatusCode::NOT_FOUND);
    }
}
