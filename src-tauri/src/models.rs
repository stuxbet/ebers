use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Row};
use tokio::sync::Mutex;

/// Type alias for the database pool state
pub type DbState = Mutex<SqlitePool>;

/// Represents a detection record in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionRecord {
    pub id: Option<i64>,
    pub uuid: String,
    pub port: String,
    pub baud_rate: i32,
    pub collection_duration_ms: i64,
    pub detection_result: Option<String>,
    pub confidence: Option<f64>,
    pub raw_response: Option<String>,
    pub status: String, // "pending", "success", "error"
    pub error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl DetectionRecord {
    /// Create a new detection record with pending status
    pub fn new_pending(
        uuid: String,
        port: String,
        baud_rate: i32,
        collection_duration_ms: i64,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: None,
            uuid,
            port,
            baud_rate,
            collection_duration_ms,
            detection_result: None,
            confidence: None,
            raw_response: None,
            status: "pending".to_string(),
            error_message: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    /// Mark the detection as successful
    pub fn mark_success(
        &mut self,
        detection_result: String,
        confidence: f64,
        raw_response: String,
    ) {
        self.detection_result = Some(detection_result);
        self.confidence = Some(confidence);
        self.raw_response = Some(raw_response);
        self.status = "success".to_string();
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Mark the detection as failed
    pub fn mark_error(&mut self, error_message: String) {
        self.status = "error".to_string();
        self.error_message = Some(error_message);
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

/// Database helper functions
pub struct Database;

impl Database {
    /// Insert a new detection record
    pub async fn insert_detection(
        pool: &SqlitePool,
        record: &DetectionRecord,
    ) -> Result<i64, String> {
        let result = sqlx::query(
            "INSERT INTO detections (uuid, port, baud_rate, collection_duration_ms,
             detection_result, confidence, raw_response, status, error_message,
             created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&record.uuid)
        .bind(&record.port)
        .bind(record.baud_rate)
        .bind(record.collection_duration_ms)
        .bind(&record.detection_result)
        .bind(record.confidence)
        .bind(&record.raw_response)
        .bind(&record.status)
        .bind(&record.error_message)
        .bind(&record.created_at)
        .bind(&record.updated_at)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to insert detection: {}", e))?;

        Ok(result.last_insert_rowid())
    }

    /// Update an existing detection record by UUID
    pub async fn update_detection(
        pool: &SqlitePool,
        record: &DetectionRecord,
    ) -> Result<(), String> {
        sqlx::query(
            "UPDATE detections SET
             detection_result = ?, confidence = ?, raw_response = ?,
             status = ?, error_message = ?, updated_at = ?
             WHERE uuid = ?",
        )
        .bind(&record.detection_result)
        .bind(record.confidence)
        .bind(&record.raw_response)
        .bind(&record.status)
        .bind(&record.error_message)
        .bind(&record.updated_at)
        .bind(&record.uuid)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update detection: {}", e))?;

        Ok(())
    }

    /// Get a detection record by UUID
    pub async fn get_detection_by_uuid(
        pool: &SqlitePool,
        uuid: &str,
    ) -> Result<Option<DetectionRecord>, String> {
        let result = sqlx::query(
            "SELECT id, uuid, port, baud_rate, collection_duration_ms,
             detection_result, confidence, raw_response, status, error_message,
             created_at, updated_at
             FROM detections WHERE uuid = ?",
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to fetch detection: {}", e))?;

        Ok(result.map(|row| DetectionRecord {
            id: Some(row.get("id")),
            uuid: row.get("uuid"),
            port: row.get("port"),
            baud_rate: row.get("baud_rate"),
            collection_duration_ms: row.get("collection_duration_ms"),
            detection_result: row.get("detection_result"),
            confidence: row.get("confidence"),
            raw_response: row.get("raw_response"),
            status: row.get("status"),
            error_message: row.get("error_message"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }))
    }

    /// Get all detections, ordered by created_at descending
    pub async fn get_all_detections(pool: &SqlitePool) -> Result<Vec<DetectionRecord>, String> {
        let results = sqlx::query(
            "SELECT id, uuid, port, baud_rate, collection_duration_ms,
             detection_result, confidence, raw_response, status, error_message,
             created_at, updated_at
             FROM detections ORDER BY created_at DESC",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch detections: {}", e))?;

        Ok(results
            .into_iter()
            .map(|row| DetectionRecord {
                id: Some(row.get("id")),
                uuid: row.get("uuid"),
                port: row.get("port"),
                baud_rate: row.get("baud_rate"),
                collection_duration_ms: row.get("collection_duration_ms"),
                detection_result: row.get("detection_result"),
                confidence: row.get("confidence"),
                raw_response: row.get("raw_response"),
                status: row.get("status"),
                error_message: row.get("error_message"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect())
    }

    /// Get detections by status
    pub async fn get_detections_by_status(
        pool: &SqlitePool,
        status: &str,
    ) -> Result<Vec<DetectionRecord>, String> {
        let results = sqlx::query(
            "SELECT id, uuid, port, baud_rate, collection_duration_ms,
             detection_result, confidence, raw_response, status, error_message,
             created_at, updated_at
             FROM detections WHERE status = ? ORDER BY created_at DESC",
        )
        .bind(status)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch detections: {}", e))?;

        Ok(results
            .into_iter()
            .map(|row| DetectionRecord {
                id: Some(row.get("id")),
                uuid: row.get("uuid"),
                port: row.get("port"),
                baud_rate: row.get("baud_rate"),
                collection_duration_ms: row.get("collection_duration_ms"),
                detection_result: row.get("detection_result"),
                confidence: row.get("confidence"),
                raw_response: row.get("raw_response"),
                status: row.get("status"),
                error_message: row.get("error_message"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect())
    }
}
