use rocket::http::{ContentType, Header, Status};
use rocket::local::blocking::Client;
use std::sync::Arc;

fn test_client() -> (Client, String) {
    let db = Arc::new(private_dashboard::db::Db::new(":memory:").unwrap());
    let key = format!("dash_test_{}", uuid::Uuid::new_v4().simple());
    db.set_manage_key(&key);

    let cors = rocket_cors::CorsOptions::default()
        .allowed_origins(rocket_cors::AllowedOrigins::all())
        .to_cors()
        .unwrap();

    let rocket = rocket::build()
        .attach(cors)
        .manage(db)
        .mount("/api/v1", rocket::routes![
            private_dashboard::routes::health,
            private_dashboard::routes::submit_stats,
            private_dashboard::routes::get_stats,
            private_dashboard::routes::get_stat_history,
        ]);

    (Client::tracked(rocket).unwrap(), key)
}

#[test]
fn test_health() {
    let (client, _) = test_client();
    let response = client.get("/api/v1/health").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body: serde_json::Value = response.into_json().unwrap();
    assert_eq!(body["status"], "ok");
    assert_eq!(body["stats_count"], 0);
    assert_eq!(body["keys_count"], 0);
}

#[test]
fn test_submit_stats_no_auth() {
    let (client, _) = test_client();
    let response = client
        .post("/api/v1/stats")
        .header(ContentType::JSON)
        .body(r#"[{"key":"test","value":42}]"#)
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_submit_stats_wrong_key() {
    let (client, _) = test_client();
    let response = client
        .post("/api/v1/stats")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", "Bearer wrong_key"))
        .body(r#"[{"key":"test","value":42}]"#)
        .dispatch();
    assert_eq!(response.status(), Status::Forbidden);
}

#[test]
fn test_submit_and_get_stats() {
    let (client, key) = test_client();

    // Submit stats
    let response = client
        .post("/api/v1/stats")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", format!("Bearer {}", key)))
        .body(r#"[{"key":"agents_discovered","value":645},{"key":"repos_count","value":7}]"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body: serde_json::Value = response.into_json().unwrap();
    assert_eq!(body["accepted"], 2);

    // Get all stats
    let response = client.get("/api/v1/stats").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body: serde_json::Value = response.into_json().unwrap();
    let stats = body["stats"].as_array().unwrap();
    assert_eq!(stats.len(), 2);

    // Check first stat
    let agents = stats.iter().find(|s| s["key"] == "agents_discovered").unwrap();
    assert_eq!(agents["current"], 645.0);
    assert_eq!(agents["label"], "Agents Discovered");
}

#[test]
fn test_submit_empty_array() {
    let (client, key) = test_client();
    let response = client
        .post("/api/v1/stats")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", format!("Bearer {}", key)))
        .body("[]")
        .dispatch();
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn test_submit_with_metadata() {
    let (client, key) = test_client();
    let response = client
        .post("/api/v1/stats")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", format!("Bearer {}", key)))
        .body(r#"[{"key":"test_metric","value":100,"metadata":{"source":"manual"}}]"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body: serde_json::Value = response.into_json().unwrap();
    assert_eq!(body["accepted"], 1);
}

#[test]
fn test_get_stat_history() {
    let (client, key) = test_client();

    // Submit a stat
    client
        .post("/api/v1/stats")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", format!("Bearer {}", key)))
        .body(r#"[{"key":"test_key","value":10}]"#)
        .dispatch();

    // Get history
    let response = client.get("/api/v1/stats/test_key?period=24h").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body: serde_json::Value = response.into_json().unwrap();
    assert_eq!(body["key"], "test_key");
    let points = body["points"].as_array().unwrap();
    assert_eq!(points.len(), 1);
    assert_eq!(points[0]["value"], 10.0);
}

#[test]
fn test_get_stat_history_invalid_period() {
    let (client, _) = test_client();
    let response = client.get("/api/v1/stats/test_key?period=invalid").dispatch();
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn test_health_after_data() {
    let (client, key) = test_client();

    client
        .post("/api/v1/stats")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", format!("Bearer {}", key)))
        .body(r#"[{"key":"a","value":1},{"key":"b","value":2}]"#)
        .dispatch();

    let response = client.get("/api/v1/health").dispatch();
    let body: serde_json::Value = response.into_json().unwrap();
    assert_eq!(body["stats_count"], 2);
    assert_eq!(body["keys_count"], 2);
}

#[test]
fn test_submit_skips_invalid_keys() {
    let (client, key) = test_client();
    let response = client
        .post("/api/v1/stats")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", format!("Bearer {}", key)))
        .body(r#"[{"key":"","value":1},{"key":"valid","value":2}]"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body: serde_json::Value = response.into_json().unwrap();
    assert_eq!(body["accepted"], 1);
}

#[test]
fn test_multiple_submits_same_key() {
    let (client, key) = test_client();

    // Submit first value
    client
        .post("/api/v1/stats")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", format!("Bearer {}", key)))
        .body(r#"[{"key":"counter","value":10}]"#)
        .dispatch();

    // Submit second value
    client
        .post("/api/v1/stats")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", format!("Bearer {}", key)))
        .body(r#"[{"key":"counter","value":20}]"#)
        .dispatch();

    // Latest should be 20
    let response = client.get("/api/v1/stats").dispatch();
    let body: serde_json::Value = response.into_json().unwrap();
    let stats = body["stats"].as_array().unwrap();
    assert_eq!(stats.len(), 1);
    assert_eq!(stats[0]["current"], 20.0);

    // History should have both points
    let response = client.get("/api/v1/stats/counter?period=24h").dispatch();
    let body: serde_json::Value = response.into_json().unwrap();
    let points = body["points"].as_array().unwrap();
    assert_eq!(points.len(), 2);
}

#[test]
fn test_get_stats_empty() {
    let (client, _) = test_client();
    let response = client.get("/api/v1/stats").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body: serde_json::Value = response.into_json().unwrap();
    assert_eq!(body["stats"].as_array().unwrap().len(), 0);
}

#[test]
fn test_stat_trends_structure() {
    let (client, key) = test_client();

    client
        .post("/api/v1/stats")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", format!("Bearer {}", key)))
        .body(r#"[{"key":"metric","value":100}]"#)
        .dispatch();

    let response = client.get("/api/v1/stats").dispatch();
    let body: serde_json::Value = response.into_json().unwrap();
    let stat = &body["stats"][0];

    // Trends should have all time windows
    assert!(stat["trends"]["24h"].is_object());
    assert!(stat["trends"]["7d"].is_object());
    assert!(stat["trends"]["30d"].is_object());
    assert!(stat["trends"]["90d"].is_object());

    // Each trend should have end = current
    assert_eq!(stat["trends"]["24h"]["end"], 100.0);
}
