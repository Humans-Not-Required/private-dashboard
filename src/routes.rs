use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::State;
use std::sync::Arc;
use chrono::{Utc, Duration};

use crate::db::Db;
use crate::auth::ManageKey;
use crate::models::*;

#[get("/health")]
pub fn health(db: &State<Arc<Db>>) -> Json<HealthResponse> {
    let keys = db.get_all_keys();
    Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        stats_count: db.get_stat_count(),
        keys_count: keys.len(),
    })
}

#[post("/stats", format = "json", data = "<stats>")]
pub fn submit_stats(
    db: &State<Arc<Db>>,
    auth: ManageKey,
    stats: Json<Vec<StatInput>>,
) -> Result<Json<StatSubmitResponse>, (Status, Json<serde_json::Value>)> {
    // Validate manage key
    let expected = db.get_manage_key().unwrap_or_default();
    if auth.0 != expected {
        return Err((
            Status::Forbidden,
            Json(serde_json::json!({"error": "Invalid manage key"})),
        ));
    }

    if stats.is_empty() {
        return Err((
            Status::BadRequest,
            Json(serde_json::json!({"error": "Empty stats array"})),
        ));
    }

    if stats.len() > 100 {
        return Err((
            Status::BadRequest,
            Json(serde_json::json!({"error": "Too many stats (max 100)"})),
        ));
    }

    let now = Utc::now().to_rfc3339();
    let mut accepted = 0;

    for stat in stats.iter() {
        if stat.key.is_empty() || stat.key.len() > 100 {
            continue;
        }
        let meta = stat.metadata.as_ref().map(|m| m.to_string());
        db.insert_stat(&stat.key, stat.value, &now, meta.as_deref());
        accepted += 1;
    }

    Ok(Json(StatSubmitResponse { accepted }))
}

#[get("/stats")]
pub fn get_stats(db: &State<Arc<Db>>) -> Json<StatsResponse> {
    let latest = db.get_latest_stats();
    let now = Utc::now();

    let stats: Vec<StatSummary> = latest.iter().map(|s| {
        let trends = Trends {
            h24: compute_trend(db, &s.key, s.value, now - Duration::hours(24)),
            d7: compute_trend(db, &s.key, s.value, now - Duration::days(7)),
            d30: compute_trend(db, &s.key, s.value, now - Duration::days(30)),
            d90: compute_trend(db, &s.key, s.value, now - Duration::days(90)),
        };
        let sparkline = db.get_sparkline(&s.key, &(now - Duration::hours(24)).to_rfc3339(), 12);

        StatSummary {
            key: s.key.clone(),
            label: key_label(&s.key),
            current: s.value,
            trends,
            sparkline_24h: sparkline,
            last_updated: s.recorded_at.clone(),
        }
    }).collect();

    Json(StatsResponse { stats })
}

#[get("/stats/<key>?<period>")]
pub fn get_stat_history(
    db: &State<Arc<Db>>,
    key: &str,
    period: Option<&str>,
) -> Result<Json<StatHistoryResponse>, (Status, Json<serde_json::Value>)> {
    let now = Utc::now();
    let since = match period.unwrap_or("24h") {
        "24h" => now - Duration::hours(24),
        "7d" => now - Duration::days(7),
        "30d" => now - Duration::days(30),
        "90d" => now - Duration::days(90),
        _ => {
            return Err((
                Status::BadRequest,
                Json(serde_json::json!({"error": "Invalid period. Use 24h, 7d, 30d, or 90d"})),
            ));
        }
    };

    let points = db.get_stat_history(key, &since.to_rfc3339());
    Ok(Json(StatHistoryResponse {
        key: key.to_string(),
        points: points.iter().map(|p| StatPointOut {
            value: p.value,
            recorded_at: p.recorded_at.clone(),
        }).collect(),
    }))
}

fn compute_trend(db: &Db, key: &str, current: f64, since: chrono::DateTime<Utc>) -> TrendData {
    let start = db.get_stat_at_time(key, &since.to_rfc3339());
    match start {
        Some(s) if s != 0.0 => TrendData {
            start: Some(s),
            end: current,
            change: Some(current - s),
            pct: Some(((current - s) / s) * 100.0),
        },
        Some(s) => TrendData {
            start: Some(s),
            end: current,
            change: Some(current - s),
            pct: None,
        },
        None => TrendData {
            start: None,
            end: current,
            change: None,
            pct: None,
        },
    }
}
