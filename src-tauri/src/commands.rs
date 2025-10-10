use crate::db_orm::Database;
use crate::models::{
    DbState, DetectionResult, Patient, Test, TestStatus, TestType, TestWithPatient,
};
use chrono::NaiveDate;
use serde::Deserialize;
use tauri::State;
use uuid::Uuid;

// ============================================================================
// SETTINGS COMMANDS
// ============================================================================

#[tauri::command]
pub async fn save_setting(
    db_state: State<'_, DbState>,
    key: String,
    value: String,
) -> Result<(), String> {
    let db = db_state.lock().await;
    Database::save_setting(&*db, key.clone(), value.clone()).await?;
    println!("Saved setting: {} = {}", key, value);
    Ok(())
}

#[tauri::command]
pub async fn get_setting(
    db_state: State<'_, DbState>,
    key: String,
) -> Result<Option<String>, String> {
    let db = db_state.lock().await;
    Database::get_setting(&*db, key).await
}

// ============================================================================
// PATIENT COMMANDS
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreatePatientRequest {
    pub first_name: String,
    pub last_name: String,
    #[serde(default, deserialize_with = "deserialize_optional_date")]
    pub date_of_birth: Option<NaiveDate>,
    #[serde(default)]
    pub patient_id_number: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

/// Custom deserializer for optional date from string
fn deserialize_optional_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(date_str) => NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn create_patient(
    db_state: State<'_, DbState>,
    patient_data: CreatePatientRequest,
) -> Result<Patient, String> {
    println!("create_patient command called");
    let pool = db_state.lock().await;

    let mut patient = Patient::new(
        patient_data.first_name,
        patient_data.last_name,
        patient_data.date_of_birth,
        patient_data.patient_id_number,
        patient_data.email,
        patient_data.phone,
        patient_data.notes,
    );

    let id = Database::insert_patient(&*pool, &patient).await?;
    patient.id = Some(id);

    println!("Created patient with id: {}", id);
    Ok(patient)
}

#[tauri::command]
pub async fn get_patient_by_uuid(
    db_state: State<'_, DbState>,
    uuid: Uuid,
) -> Result<Option<Patient>, String> {
    let pool = db_state.lock().await;
    Database::get_patient_by_uuid(&*pool, &uuid.to_string()).await
}

#[tauri::command]
pub async fn get_all_patients(db_state: State<'_, DbState>) -> Result<Vec<Patient>, String> {
    println!("get_all_patients command called");
    let pool = db_state.lock().await;
    let result = Database::get_all_patients(&*pool).await;
    match &result {
        Ok(patients) => println!("Successfully fetched {} patients", patients.len()),
        Err(e) => println!("Error fetching patients: {}", e),
    }
    result
}

// ============================================================================
// TEST COMMANDS
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateTestRequest {
    pub patient_uuid: Uuid,
    pub test_type: TestType,
    pub device_id: Option<String>,
    pub firmware_version: Option<String>,
}

#[tauri::command]
pub async fn create_test(
    db_state: State<'_, DbState>,
    test_data: CreateTestRequest,
) -> Result<Test, String> {
    println!(
        "create_test command called for patient: {}",
        test_data.patient_uuid
    );
    let pool = db_state.lock().await;

    // Get patient by UUID
    let patient = Database::get_patient_by_uuid(&*pool, &test_data.patient_uuid.to_string())
        .await?
        .ok_or_else(|| format!("Patient not found: {}", test_data.patient_uuid))?;

    let patient_id = patient.id.ok_or("Patient has no ID")?;

    let mut test = Test::new_pending(
        patient_id,
        test_data.test_type,
        test_data.device_id,
        test_data.firmware_version,
    );

    let id = Database::insert_test(&*pool, &test).await?;
    test.id = Some(id);

    println!("Created test with id: {} and uuid: {}", id, test.uuid);
    Ok(test)
}

#[tauri::command]
pub async fn get_test_by_uuid(
    db_state: State<'_, DbState>,
    uuid: Uuid,
) -> Result<Option<Test>, String> {
    let pool = db_state.lock().await;
    Database::get_test_by_uuid(&*pool, &uuid.to_string()).await
}

#[tauri::command]
pub async fn get_all_tests(db_state: State<'_, DbState>) -> Result<Vec<TestWithPatient>, String> {
    println!("get_all_tests command called");
    let pool = db_state.lock().await;
    let result = Database::get_all_tests_with_patients(&*pool).await;
    match &result {
        Ok(tests) => println!("Successfully fetched {} tests", tests.len()),
        Err(e) => println!("Error fetching tests: {}", e),
    }
    result
}

#[tauri::command]
pub async fn update_test_status(
    db_state: State<'_, DbState>,
    test_uuid: Uuid,
    status: TestStatus,
) -> Result<(), String> {
    println!("update_test_status called: {} -> {:?}", test_uuid, status);
    let pool = db_state.lock().await;

    let mut test = Database::get_test_by_uuid(&*pool, &test_uuid.to_string())
        .await?
        .ok_or_else(|| format!("Test not found: {}", test_uuid))?;

    match status {
        TestStatus::InProgress => test.mark_in_progress(),
        TestStatus::Error => test.mark_error("Test failed".to_string()),
        _ => {
            test.status = status;
            test.touch();
        }
    }

    Database::update_test(&*pool, &test).await
}

#[derive(Debug, Deserialize)]
pub struct CompleteTestRequest {
    pub test_uuid: Uuid,
    pub detection_result: DetectionResult,
    pub confidence: f64,
    pub raw_response: String,
}

#[tauri::command]
pub async fn complete_test(
    db_state: State<'_, DbState>,
    data: CompleteTestRequest,
) -> Result<(), String> {
    println!("complete_test called for: {}", data.test_uuid);
    let pool = db_state.lock().await;

    let mut test = Database::get_test_by_uuid(&*pool, &data.test_uuid.to_string())
        .await?
        .ok_or_else(|| format!("Test not found: {}", data.test_uuid))?;

    test.mark_completed(data.detection_result, data.confidence, data.raw_response);

    Database::update_test(&*pool, &test).await
}
