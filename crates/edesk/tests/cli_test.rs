use assert_cmd::Command;
use httpmock::prelude::*;
use predicates::prelude::*;
use serde_json::json;

/// Build an `edesk` command pointed at a mock server with a dummy token.
fn edesk(server: &MockServer) -> Command {
    let mut cmd = Command::cargo_bin("edesk").unwrap();
    cmd.env("EDESK_TOKEN", "test-token")
        .env("EDESK_BASE_URL", server.base_url())
        .env("NO_COLOR", "1");
    cmd
}

#[test]
fn whoami_renders_user() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/whoami");
        then.status(200).json_body(
            json!({"data": {"user": {"id": 9, "name": "Test", "email": "t@example.com"}}}),
        );
    });

    edesk(&server)
        .arg("whoami")
        .assert()
        .success()
        .stdout(predicate::str::contains("t@example.com"));
}

#[test]
fn ticket_list_outputs_tsv_when_piped() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/tickets");
        then.status(200).json_body(json!({
            "data": [
                {"id": 1, "subject": "Where is my order", "status": "Open", "type": "OrderQuery",
                 "channel_id": 5, "contact_id": 10, "last_updated_at": "2026-01-01 10:00:00"},
                {"id": 2, "subject": "Refund please", "status": "Closed", "type": "Refund",
                 "channel_id": 5, "contact_id": 11, "last_updated_at": null}
            ],
            "paginator": {"currentPage": 1, "itemsPerPage": 20, "totalItemsCount": 2}
        }));
    });

    edesk(&server)
        .args(["ticket", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1\tWhere is my order\tOpen"))
        .stdout(predicate::str::contains("2\tRefund please\tClosed"));
}

#[test]
fn ticket_list_json_outputs_raw_array() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/tickets");
        then.status(200).json_body(json!({
            "data": [{"id": 42, "subject": "hi", "status": "Open"}],
            "paginator": {"currentPage": 1, "itemsPerPage": 20, "totalItemsCount": 1}
        }));
    });

    let output = edesk(&server)
        .args(["ticket", "list", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let parsed: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(
        parsed,
        json!([{"id": 42, "subject": "hi", "status": "Open"}])
    );
}

#[test]
fn jq_filter_extracts_values() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/tickets");
        then.status(200).json_body(json!({
            "data": [{"id": 1, "status": "Open"}, {"id": 2, "status": "Closed"}],
            "paginator": {"currentPage": 1, "itemsPerPage": 20, "totalItemsCount": 2}
        }));
    });

    edesk(&server)
        .args([
            "ticket",
            "list",
            "--jq",
            ".[] | select(.status == \"Open\") | .id",
        ])
        .assert()
        .success()
        .stdout(predicate::str::diff("1\n"));
}

#[test]
fn fields_projects_json_output() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/users");
        then.status(200).json_body(json!({
            "data": [{"id": 1, "name": "A", "email": "a@x.y", "role": "agent"}],
            "paginator": {"currentPage": 1, "itemsPerPage": 20, "totalItemsCount": 1}
        }));
    });

    let output = edesk(&server)
        .args(["user", "list", "--json", "--fields", "id,email"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let parsed: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(parsed, json!([{"id": 1, "email": "a@x.y"}]));
}

#[test]
fn list_paginates_until_limit() {
    let server = MockServer::start();
    let page1 = server.mock(|when, then| {
        when.method(GET).path("/tags").query_param("page", "1");
        then.status(200).json_body(json!({
            "data": (1..=100).map(|i| json!({"id": i, "name": format!("tag{i}")})).collect::<Vec<_>>(),
            "paginator": {"currentPage": 1, "itemsPerPage": 100, "totalItemsCount": 150}
        }));
    });
    let page2 = server.mock(|when, then| {
        when.method(GET).path("/tags").query_param("page", "2");
        then.status(200).json_body(json!({
            "data": (101..=150).map(|i| json!({"id": i, "name": format!("tag{i}")})).collect::<Vec<_>>(),
            "paginator": {"currentPage": 2, "itemsPerPage": 100, "totalItemsCount": 150}
        }));
    });

    let output = edesk(&server)
        .args(["tag", "list", "--all", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let parsed: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(parsed.as_array().unwrap().len(), 150);
    page1.assert();
    page2.assert();
}

#[test]
fn delete_without_confirmation_fails_outside_tty() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(DELETE).path("/tags/1");
        then.status(200)
            .json_body(json!({"data": {"ok": true, "message": "deleted"}}));
    });

    edesk(&server)
        .args(["tag", "delete", "1"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("--yes"));
    mock.assert_hits(0);
}

#[test]
fn delete_with_yes_succeeds() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(DELETE).path("/tags/1");
        then.status(200)
            .json_body(json!({"data": {"ok": true, "message": "Tag deleted"}}));
    });

    edesk(&server)
        .args(["tag", "delete", "1", "--yes"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Tag deleted"));
    mock.assert();
}

#[test]
fn invalid_token_exits_with_code_4() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/whoami");
        then.status(403).json_body(json!({
            "error": {"httpCode": 403, "message": "You are not authorized to access this", "details": "Invalid token."}
        }));
    });

    edesk(&server)
        .arg("whoami")
        .assert()
        .failure()
        .code(4)
        .stderr(predicate::str::contains("edesk auth login"));
}

#[test]
fn validation_errors_list_fields() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST).path("/tags");
        then.status(400).json_body(json!({
            "error": {"httpCode": 400, "message": "Validation failed",
                      "details": {"tag_group_id": {"errorCode": 4002}}}
        }));
    });

    edesk(&server)
        .args(["tag", "create", "--name", "x", "--group", "999"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("tag_group_id"))
        .stderr(predicate::str::contains("object not found"));
}

#[test]
fn api_command_prints_envelope() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/whoami");
        then.status(200)
            .json_body(json!({"data": {"user": {"id": 1}}}));
    });

    let output = edesk(&server)
        .args(["api", "/whoami"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let parsed: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(parsed, json!({"data": {"user": {"id": 1}}}));
}

#[test]
fn api_command_paginate_combines_pages() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/channels").query_param("page", "1");
        then.status(200).json_body(json!({
            "data": [{"id": 1}],
            "paginator": {"currentPage": 1, "itemsPerPage": 1, "totalItemsCount": 2}
        }));
    });
    server.mock(|when, then| {
        when.method(GET).path("/channels").query_param("page", "2");
        then.status(200).json_body(json!({
            "data": [{"id": 2}],
            "paginator": {"currentPage": 2, "itemsPerPage": 1, "totalItemsCount": 2}
        }));
    });

    let output = edesk(&server)
        .args(["api", "/channels", "--paginate"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let parsed: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(parsed, json!([{"id": 1}, {"id": 2}]));
}

#[test]
fn message_create_sends_typed_body() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/messages")
            .json_body_partial(r#"{"ticket_id": 7, "body": "hello", "type": "Note"}"#);
        then.status(200)
            .json_body(json!({"data": {"id": 1, "type": "Note", "body": "hello"}}));
    });

    edesk(&server)
        .args([
            "message", "create", "--ticket", "7", "--body", "hello", "--type", "Note",
        ])
        .assert()
        .success();
    mock.assert();
}

#[test]
fn completion_generates_script() {
    Command::cargo_bin("edesk")
        .unwrap()
        .args(["completion", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_edesk"));
}

#[test]
fn help_shows_subcommands() {
    Command::cargo_bin("edesk")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("ticket"))
        .stdout(predicate::str::contains("auth"))
        .stdout(predicate::str::contains("api"));
}

#[test]
fn global_fields_flag_works_alongside_custom_field_flag() {
    // Regression: the custom-field flag's arg ID used to collide with the
    // global --fields, making clap reject --fields on ticket subcommands.
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(PUT).path("/tickets/7/data");
        then.status(200)
            .json_body(json!({"data": {"id": 7, "subject": "s", "status": "Open"}}));
    });

    let output = edesk(&server)
        .args([
            "ticket",
            "update-data",
            "7",
            "-f",
            "a=b",
            "--json",
            "--fields",
            "id,status",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let parsed: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(parsed, json!({"id": 7, "status": "Open"}));
}

#[test]
fn api_command_honors_fields_projection() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/whoami");
        then.status(200)
            .json_body(json!({"data": {"user": {"id": 1, "email": "a@b.c"}}}));
    });

    let output = edesk(&server)
        .args(["api", "/whoami", "--fields", "data.user.email"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let parsed: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(parsed, json!({"data.user.email": "a@b.c"}));
}

#[test]
fn upgrade_check_reports_newer_release() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET)
            .path("/repos/Hamelyn-SL/edesk-cli/releases/latest");
        then.status(200).json_body(json!({"tag_name": "v99.0.0"}));
    });

    Command::cargo_bin("edesk")
        .unwrap()
        .env("EDESK_UPDATE_CHECK_URL", server.base_url())
        .args(["upgrade", "--check"])
        .assert()
        .success()
        .stdout(predicate::str::contains("99.0.0"));
}

#[test]
fn upgrade_check_reports_up_to_date() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET)
            .path("/repos/Hamelyn-SL/edesk-cli/releases/latest");
        then.status(200)
            .json_body(json!({"tag_name": format!("v{}", env!("CARGO_PKG_VERSION"))}));
    });

    Command::cargo_bin("edesk")
        .unwrap()
        .env("EDESK_UPDATE_CHECK_URL", server.base_url())
        .args(["upgrade", "--check"])
        .assert()
        .success()
        .stdout(predicate::str::contains("up to date"));
}

#[test]
fn help_includes_install_instructions() {
    Command::cargo_bin("edesk")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Installation & updates"))
        .stdout(predicate::str::contains(
            "brew install Hamelyn-SL/tap/edesk",
        ))
        .stdout(predicate::str::contains("edesk upgrade"));
}
