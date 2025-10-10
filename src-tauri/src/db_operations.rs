use crate::models::{DetectionResult, Patient, Test, TestStatus, TestType, TestWithPatient};
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{sqlite::SqlitePool, Row};
use uuid::Uuid;

// ============================================================================
// DATABASE HELPER FUNCTIONS
// ============================================================================

pub struct Database;

impl Database {
    // ------------------------------------------------------------------------
    // PATIENT OPERATIONS
    // ------------------------------------------------------------------------

    /// Insert a new patient record
    pub async fn insert_patient(pool: &SqlitePool, patient: &Patient) -> Result<i64, String> {
        let result = sqlx::query(
            "INSERT INTO patients (uuid, first_name, last_name, date_of_birth,
             patient_id_number, email, phone, notes, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(patient.uuid.to_string())
        .bind(&patient.first_name)
        .bind(&patient.last_name)
        .bind(patient.date_of_birth.map(|d| d.to_string()))
        .bind(&patient.patient_id_number)
        .bind(&patient.email)
        .bind(&patient.phone)
        .bind(&patient.notes)
        .bind(patient.created_at.to_rfc3339())
        .bind(patient.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to insert patient: {}", e))?;

        Ok(result.last_insert_rowid())
    }

    /// Get a patient by UUID
    pub async fn get_patient_by_uuid(
        pool: &SqlitePool,
        uuid: &str,
    ) -> Result<Option<Patient>, String> {
        let result = sqlx::query(
            "SELECT id, uuid, first_name, last_name, date_of_birth,
             patient_id_number, email, phone, notes, created_at, updated_at
             FROM patients WHERE uuid = ?",
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to fetch patient: {}", e))?;

        Ok(result.map(|row| {
            let uuid_str: String = row.get("uuid");
            let dob_str: Option<String> = row.get("date_of_birth");
            let created_str: String = row.get("created_at");
            let updated_str: String = row.get("updated_at");

            Patient {
                id: Some(row.get("id")),
                uuid: Uuid::parse_str(&uuid_str).unwrap_or_else(|_| Uuid::new_v4()),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                date_of_birth: dob_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
                patient_id_number: row.get("patient_id_number"),
                email: row.get("email"),
                phone: row.get("phone"),
                notes: row.get("notes"),
                created_at: DateTime::parse_from_rfc3339(&created_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&updated_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }
        }))
    }

    /// Get a patient by ID
    pub async fn get_patient_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Patient>, String> {
        let result = sqlx::query(
            "SELECT id, uuid, first_name, last_name, date_of_birth,
             patient_id_number, email, phone, notes, created_at, updated_at
             FROM patients WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to fetch patient: {}", e))?;

        Ok(result.map(|row| {
            let uuid_str: String = row.get("uuid");
            let dob_str: Option<String> = row.get("date_of_birth");
            let created_str: String = row.get("created_at");
            let updated_str: String = row.get("updated_at");

            Patient {
                id: Some(row.get("id")),
                uuid: Uuid::parse_str(&uuid_str).unwrap_or_else(|_| Uuid::new_v4()),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                date_of_birth: dob_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
                patient_id_number: row.get("patient_id_number"),
                email: row.get("email"),
                phone: row.get("phone"),
                notes: row.get("notes"),
                created_at: DateTime::parse_from_rfc3339(&created_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&updated_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }
        }))
    }

    /// Get all patients, ordered by created_at descending
    pub async fn get_all_patients(pool: &SqlitePool) -> Result<Vec<Patient>, String> {
        let results = sqlx::query(
            "SELECT id, uuid, first_name, last_name, date_of_birth,
             patient_id_number, email, phone, notes, created_at, updated_at
             FROM patients ORDER BY created_at DESC",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch patients: {}", e))?;

        Ok(results
            .into_iter()
            .map(|row| {
                let uuid_str: String = row.get("uuid");
                let dob_str: Option<String> = row.get("date_of_birth");
                let created_str: String = row.get("created_at");
                let updated_str: String = row.get("updated_at");

                Patient {
                    id: Some(row.get("id")),
                    uuid: Uuid::parse_str(&uuid_str).unwrap_or_else(|_| Uuid::new_v4()),
                    first_name: row.get("first_name"),
                    last_name: row.get("last_name"),
                    date_of_birth: dob_str
                        .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
                    patient_id_number: row.get("patient_id_number"),
                    email: row.get("email"),
                    phone: row.get("phone"),
                    notes: row.get("notes"),
                    created_at: DateTime::parse_from_rfc3339(&created_str)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: DateTime::parse_from_rfc3339(&updated_str)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                }
            })
            .collect())
    }

    // ------------------------------------------------------------------------
    // TEST OPERATIONS
    // ------------------------------------------------------------------------

    /// Insert a new test record
    pub async fn insert_test(pool: &SqlitePool, test: &Test) -> Result<i64, String> {
        let result = sqlx::query(
            "INSERT INTO tests (uuid, patient_id, test_type, device_id, firmware_version,
             detection_result, confidence, raw_response, status, error_message,
             created_at, updated_at, completed_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(test.uuid.to_string())
        .bind(test.patient_id)
        .bind(test.test_type.as_str())
        .bind(&test.device_id)
        .bind(&test.firmware_version)
        .bind(test.detection_result.as_ref().map(|r| r.as_str()))
        .bind(test.confidence)
        .bind(&test.raw_response)
        .bind(test.status.as_str())
        .bind(&test.error_message)
        .bind(test.created_at.to_rfc3339())
        .bind(test.updated_at.to_rfc3339())
        .bind(test.completed_at.map(|dt| dt.to_rfc3339()))
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to insert test: {}", e))?;

        Ok(result.last_insert_rowid())
    }

    /// Update an existing test record by UUID
    pub async fn update_test(pool: &SqlitePool, test: &Test) -> Result<(), String> {
        sqlx::query(
            "UPDATE tests SET
             device_id = ?, firmware_version = ?, detection_result = ?, confidence = ?,
             raw_response = ?, status = ?, error_message = ?,
             updated_at = ?, completed_at = ?
             WHERE uuid = ?",
        )
        .bind(&test.device_id)
        .bind(&test.firmware_version)
        .bind(test.detection_result.as_ref().map(|r| r.as_str()))
        .bind(test.confidence)
        .bind(&test.raw_response)
        .bind(test.status.as_str())
        .bind(&test.error_message)
        .bind(test.updated_at.to_rfc3339())
        .bind(test.completed_at.map(|dt| dt.to_rfc3339()))
        .bind(test.uuid.to_string())
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update test: {}", e))?;

        Ok(())
    }

    /// Get a test by UUID
    pub async fn get_test_by_uuid(pool: &SqlitePool, uuid: &str) -> Result<Option<Test>, String> {
        let result = sqlx::query(
            "SELECT id, uuid, patient_id, test_type, device_id, firmware_version,
             detection_result, confidence, raw_response, status, error_message,
             created_at, updated_at, completed_at
             FROM tests WHERE uuid = ?",
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to fetch test: {}", e))?;

        Ok(result.map(|row| {
            let uuid_str: String = row.get("uuid");
            let test_type_str: String = row.get("test_type");
            let detection_result_str: Option<String> = row.get("detection_result");
            let status_str: String = row.get("status");
            let created_str: String = row.get("created_at");
            let updated_str: String = row.get("updated_at");
            let completed_str: Option<String> = row.get("completed_at");

            Test {
                id: Some(row.get("id")),
                uuid: Uuid::parse_str(&uuid_str).unwrap_or_else(|_| Uuid::new_v4()),
                patient_id: row.get("patient_id"),
                test_type: TestType::from_str(&test_type_str),
                device_id: row.get("device_id"),
                firmware_version: row.get("firmware_version"),
                detection_result: detection_result_str
                    .and_then(|s| DetectionResult::from_str(&s).ok()),
                confidence: row.get("confidence"),
                raw_response: row.get("raw_response"),
                status: TestStatus::from_str(&status_str).unwrap_or(TestStatus::Error),
                error_message: row.get("error_message"),
                created_at: DateTime::parse_from_rfc3339(&created_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&updated_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                completed_at: completed_str.and_then(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .map(|dt| dt.with_timezone(&Utc))
                        .ok()
                }),
            }
        }))
    }

    /// Get all tests with patient information, ordered by created_at descending
    pub async fn get_all_tests_with_patients(
        pool: &SqlitePool,
    ) -> Result<Vec<TestWithPatient>, String> {
        let results = sqlx::query(
            "SELECT
                t.id as test_id, t.uuid as test_uuid, t.patient_id, t.test_type,
                t.device_id, t.firmware_version, t.detection_result,
                t.confidence, t.raw_response, t.status, t.error_message,
                t.created_at as test_created_at, t.updated_at as test_updated_at,
                t.completed_at,
                p.id as patient_id, p.uuid as patient_uuid, p.first_name, p.last_name,
                p.date_of_birth, p.patient_id_number, p.email, p.phone, p.notes,
                p.created_at as patient_created_at, p.updated_at as patient_updated_at
             FROM tests t
             INNER JOIN patients p ON t.patient_id = p.id
             ORDER BY t.created_at DESC",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch tests with patients: {}", e))?;

        Ok(results
            .into_iter()
            .map(|row| {
                // Parse test fields
                let test_uuid_str: String = row.get("test_uuid");
                let test_type_str: String = row.get("test_type");
                let detection_result_str: Option<String> = row.get("detection_result");
                let status_str: String = row.get("status");
                let test_created_str: String = row.get("test_created_at");
                let test_updated_str: String = row.get("test_updated_at");
                let test_completed_str: Option<String> = row.get("completed_at");

                // Parse patient fields
                let patient_uuid_str: String = row.get("patient_uuid");
                let patient_dob_str: Option<String> = row.get("date_of_birth");
                let patient_created_str: String = row.get("patient_created_at");
                let patient_updated_str: String = row.get("patient_updated_at");

                TestWithPatient {
                    test: Test {
                        id: Some(row.get("test_id")),
                        uuid: Uuid::parse_str(&test_uuid_str).unwrap_or_else(|_| Uuid::new_v4()),
                        patient_id: row.get("patient_id"),
                        test_type: TestType::from_str(&test_type_str),
                        device_id: row.get("device_id"),
                        firmware_version: row.get("firmware_version"),
                        detection_result: detection_result_str
                            .and_then(|s| DetectionResult::from_str(&s).ok()),
                        confidence: row.get("confidence"),
                        raw_response: row.get("raw_response"),
                        status: TestStatus::from_str(&status_str).unwrap_or(TestStatus::Error),
                        error_message: row.get("error_message"),
                        created_at: DateTime::parse_from_rfc3339(&test_created_str)
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(|_| Utc::now()),
                        updated_at: DateTime::parse_from_rfc3339(&test_updated_str)
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(|_| Utc::now()),
                        completed_at: test_completed_str.and_then(|s| {
                            DateTime::parse_from_rfc3339(&s)
                                .map(|dt| dt.with_timezone(&Utc))
                                .ok()
                        }),
                    },
                    patient: Patient {
                        id: Some(row.get("patient_id")),
                        uuid: Uuid::parse_str(&patient_uuid_str).unwrap_or_else(|_| Uuid::new_v4()),
                        first_name: row.get("first_name"),
                        last_name: row.get("last_name"),
                        date_of_birth: patient_dob_str
                            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
                        patient_id_number: row.get("patient_id_number"),
                        email: row.get("email"),
                        phone: row.get("phone"),
                        notes: row.get("notes"),
                        created_at: DateTime::parse_from_rfc3339(&patient_created_str)
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(|_| Utc::now()),
                        updated_at: DateTime::parse_from_rfc3339(&patient_updated_str)
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(|_| Utc::now()),
                    },
                }
            })
            .collect())
    }

    /// Get tests by patient ID
    pub async fn get_tests_by_patient_id(
        pool: &SqlitePool,
        patient_id: i64,
    ) -> Result<Vec<Test>, String> {
        let results = sqlx::query(
            "SELECT id, uuid, patient_id, test_type, device_id, firmware_version,
             detection_result, confidence, raw_response, status, error_message,
             created_at, updated_at, completed_at
             FROM tests WHERE patient_id = ? ORDER BY created_at DESC",
        )
        .bind(patient_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch tests: {}", e))?;

        Ok(results
            .into_iter()
            .map(|row| {
                let uuid_str: String = row.get("uuid");
                let test_type_str: String = row.get("test_type");
                let detection_result_str: Option<String> = row.get("detection_result");
                let status_str: String = row.get("status");
                let created_str: String = row.get("created_at");
                let updated_str: String = row.get("updated_at");
                let completed_str: Option<String> = row.get("completed_at");

                Test {
                    id: Some(row.get("id")),
                    uuid: Uuid::parse_str(&uuid_str).unwrap_or_else(|_| Uuid::new_v4()),
                    patient_id: row.get("patient_id"),
                    test_type: TestType::from_str(&test_type_str),
                    device_id: row.get("device_id"),
                    firmware_version: row.get("firmware_version"),
                    detection_result: detection_result_str
                        .and_then(|s| DetectionResult::from_str(&s).ok()),
                    confidence: row.get("confidence"),
                    raw_response: row.get("raw_response"),
                    status: TestStatus::from_str(&status_str).unwrap_or(TestStatus::Error),
                    error_message: row.get("error_message"),
                    created_at: DateTime::parse_from_rfc3339(&created_str)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: DateTime::parse_from_rfc3339(&updated_str)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    completed_at: completed_str.and_then(|s| {
                        DateTime::parse_from_rfc3339(&s)
                            .map(|dt| dt.with_timezone(&Utc))
                            .ok()
                    }),
                }
            })
            .collect())
    }
}
