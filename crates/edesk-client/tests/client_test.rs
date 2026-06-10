use httpmock::prelude::*;
use serde_json::json;

fn client(server: &MockServer) -> edesk_client::Client {
    edesk_client::Client::builder()
        .token("test-token")
        .base_url(server.base_url())
        .build()
        .unwrap()
}

#[tokio::test]
async fn whoami_decodes_envelope_and_sends_bearer_token() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/whoami")
            .header("authorization", "Bearer test-token");
        then.status(200)
            .json_body(json!({"data": {"user": {"id": 1, "email": "a@b.c"}}}));
    });

    let resp = client(&server).whoami().await.unwrap();
    mock.assert();
    assert_eq!(resp.data["user"]["email"], "a@b.c");
    assert!(resp.paginator.is_none());
}

#[tokio::test]
async fn list_parses_paginator_and_pagination_params() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/tags")
            .query_param("page", "2")
            .query_param("itemsPerPage", "50");
        then.status(200).json_body(json!({
            "data": [{"id": 1, "name": "urgent"}],
            "paginator": {"currentPage": 2, "itemsPerPage": 50, "totalItemsCount": 120}
        }));
    });

    let page = edesk_client::Page {
        page: Some(2),
        items_per_page: Some(50),
    };
    let resp = client(&server).list_tags(page).await.unwrap();
    mock.assert();
    let paginator = resp.paginator.unwrap();
    assert_eq!(paginator.current_page, 2);
    assert!(paginator.has_more());
}

#[tokio::test]
async fn list_tickets_serializes_filters_as_query_params() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/tickets")
            .query_param("filter_status_equals", "Open")
            .query_param("filter_channel_id_equals", "42")
            .query_param("order_by", "created_at");
        then.status(200)
            .json_body(json!({"data": [], "paginator": {"currentPage": 1, "itemsPerPage": 20, "totalItemsCount": 0}}));
    });

    let params = edesk_client::api::ListTicketsParams {
        status: Some("Open".into()),
        channel_id: Some(42),
        order_by: Some("created_at".into()),
        ..Default::default()
    };
    client(&server).list_tickets(&params).await.unwrap();
    mock.assert();
}

#[tokio::test]
async fn validation_error_exposes_field_codes() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST).path("/tags");
        then.status(400).json_body(json!({
            "error": {
                "httpCode": 400,
                "message": "Validation failed",
                "details": {"name": {"errorCode": 4001}, "color": {"errorCode": 4007}}
            }
        }));
    });

    let req = edesk_client::api::TagRequest {
        name: Some(String::new()),
        tag_group_id: Some(1),
        color: Some("magenta".into()),
        ..Default::default()
    };
    let err = client(&server).create_tag(&req).await.unwrap_err();
    match err {
        edesk_client::Error::Validation { field_errors, .. } => {
            assert_eq!(field_errors.len(), 2);
            let name_error = field_errors.iter().find(|f| f.field == "name").unwrap();
            assert_eq!(name_error.code, 4001);
            assert_eq!(name_error.reason(), "missing required field");
        }
        other => panic!("expected Validation, got {other:?}"),
    }
}

#[tokio::test]
async fn unauthorized_maps_to_auth_error() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/whoami");
        then.status(403).json_body(json!({
            "error": {"httpCode": 403, "message": "You are not authorized to access this", "details": "Invalid token."}
        }));
    });

    let err = client(&server).whoami().await.unwrap_err();
    assert!(matches!(err, edesk_client::Error::Auth { status: 403, .. }));
}

#[tokio::test]
async fn get_retries_on_server_error() {
    let server = MockServer::start();
    // httpmock returns mocks in definition order for equal matches; emulate a
    // flaky server by counting hits on a single mock that always fails, then
    // assert the client tried 3 times before giving up.
    let mock = server.mock(|when, then| {
        when.method(GET).path("/users");
        then.status(503)
            .json_body(json!({"error": {"httpCode": 503, "message": "unavailable"}}));
    });

    let err = client(&server)
        .list_users(edesk_client::Page::default())
        .await
        .unwrap_err();
    assert!(matches!(err, edesk_client::Error::Api { status: 503, .. }));
    mock.assert_hits(3);
}

#[tokio::test]
async fn post_does_not_retry_on_server_error() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/order-notes");
        then.status(500)
            .json_body(json!({"error": {"httpCode": 500, "message": "boom"}}));
    });

    let req = edesk_client::api::CreateOrderNoteRequest {
        sales_order_id: 1,
        text: "hi".into(),
    };
    let _ = client(&server).create_order_note(&req).await.unwrap_err();
    mock.assert_hits(1);
}

#[tokio::test]
async fn delete_returns_confirmation_payload() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(DELETE).path("/tickets/77");
        then.status(200)
            .json_body(json!({"data": {"ok": true, "message": "Ticket deleted"}}));
    });

    let resp = client(&server).delete_ticket(77).await.unwrap();
    assert_eq!(resp.data["ok"], json!(true));
    assert_eq!(resp.data["message"], "Ticket deleted");
}

#[tokio::test]
async fn non_json_error_body_is_reported() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/whoami");
        then.status(404).body("not found at the gateway");
    });

    let err = client(&server).whoami().await.unwrap_err();
    match err {
        edesk_client::Error::Api {
            status, message, ..
        } => {
            assert_eq!(status, 404);
            assert_eq!(message, "not found at the gateway");
        }
        other => panic!("expected Api, got {other:?}"),
    }
}
