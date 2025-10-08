use crate::models::{Database, DbState, DetectionRecord};
use tauri::State;

// Database commands
#[tauri::command]
pub async fn get_all_detections(
    db_state: State<'_, DbState>,
) -> Result<Vec<DetectionRecord>, String> {
    println!("get_all_detections command called");
    let pool = db_state.lock().await;
    let result = Database::get_all_detections(&*pool).await;
    match &result {
        Ok(detections) => println!("Successfully fetched {} detections", detections.len()),
        Err(e) => println!("Error fetching detections: {}", e),
    }
    result
}

#[tauri::command]
pub async fn get_detection_by_uuid(
    db_state: State<'_, DbState>,
    uuid: String,
) -> Result<Option<DetectionRecord>, String> {
    let pool = db_state.lock().await;
    Database::get_detection_by_uuid(&*pool, &uuid).await
}

#[tauri::command]
pub async fn get_detections_by_status(
    db_state: State<'_, DbState>,
    status: String,
) -> Result<Vec<DetectionRecord>, String> {
    let pool = db_state.lock().await;
    Database::get_detections_by_status(&*pool, &status).await
}

// Test command to insert sample data
#[tauri::command]
pub async fn insert_test_detection(db_state: State<'_, DbState>) -> Result<String, String> {
    use chrono::Utc;

    println!("insert_test_detection command called");
    let pool = db_state.lock().await;

    let test_record = DetectionRecord {
        id: None,
        uuid: uuid::Uuid::new_v4().to_string(),
        port: "COM3".to_string(),
        baud_rate: 9600,
        collection_duration_ms: 5000,
        detection_result: Some("Test Result - Sample Detection".to_string()),
        confidence: Some(0.95),
        raw_response: Some(r#"{"probability": 0.95, "confidence": 0.95}"#.to_string()),
        status: "success".to_string(),
        error_message: None,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    let id = Database::insert_detection(&*pool, &test_record).await?;
    println!("Inserted test detection with id: {}", id);
    Ok(format!("Inserted test detection with id: {}", id))
}
