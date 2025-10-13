use crate::entities::{patient, setting, test};
use crate::models::{Patient, Test, TestWithPatient};
use sea_orm::*;

// ============================================================================
// DATABASE ORM OPERATIONS (SeaORM)
// ============================================================================

pub struct Database;

impl Database {
    // ------------------------------------------------------------------------
    // SETTINGS OPERATIONS
    // ------------------------------------------------------------------------

    /// Save a setting (insert or replace)
    pub async fn save_setting(
        db: &DatabaseConnection,
        key: String,
        value: String,
    ) -> Result<(), String> {
        let setting_model = setting::ActiveModel {
            key: Set(key.clone()),
            value: Set(value),
        };

        // Use insert with on_conflict to handle upsert
        setting::Entity::insert(setting_model)
            .on_conflict(
                sea_query::OnConflict::column(setting::Column::Key)
                    .update_column(setting::Column::Value)
                    .to_owned(),
            )
            .exec(db)
            .await
            .map_err(|e| format!("Failed to save setting: {}", e))?;

        Ok(())
    }

    /// Get a setting by key
    pub async fn get_setting(
        db: &DatabaseConnection,
        key: String,
    ) -> Result<Option<String>, String> {
        let result = setting::Entity::find_by_id(key)
            .one(db)
            .await
            .map_err(|e| format!("Failed to get setting: {}", e))?;

        Ok(result.map(|model| model.value))
    }

    // ------------------------------------------------------------------------
    // PATIENT OPERATIONS
    // ------------------------------------------------------------------------

    /// Insert a new patient record
    pub async fn insert_patient(db: &DatabaseConnection, patient: &Patient) -> Result<i64, String> {
        let patient_model = patient::ActiveModel {
            uuid: Set(patient.uuid.clone()),
            first_name: Set(patient.first_name.clone()),
            last_name: Set(patient.last_name.clone()),
            date_of_birth: Set(patient.date_of_birth.clone()),
            patient_id_number: Set(patient.patient_id_number.clone()),
            email: Set(patient.email.clone()),
            phone: Set(patient.phone.clone()),
            notes: Set(patient.notes.clone()),
            created_at: Set(patient.created_at.clone()),
            updated_at: Set(patient.updated_at.clone()),
            ..Default::default()
        };

        let result = patient::Entity::insert(patient_model)
            .exec(db)
            .await
            .map_err(|e| format!("Failed to insert patient: {}", e))?;

        Ok(result.last_insert_id)
    }

    /// Get a patient by UUID
    pub async fn get_patient_by_uuid(
        db: &DatabaseConnection,
        uuid: &str,
    ) -> Result<Option<Patient>, String> {
        let result = patient::Entity::find()
            .filter(patient::Column::Uuid.eq(uuid))
            .one(db)
            .await
            .map_err(|e| format!("Failed to fetch patient: {}", e))?;

        Ok(result.map(|model| Patient {
            id: Some(model.id),
            uuid: model.uuid,
            first_name: model.first_name,
            last_name: model.last_name,
            date_of_birth: model.date_of_birth,
            patient_id_number: model.patient_id_number,
            email: model.email,
            phone: model.phone,
            notes: model.notes,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }))
    }

    /// Get all patients
    pub async fn get_all_patients(db: &DatabaseConnection) -> Result<Vec<Patient>, String> {
        let results = patient::Entity::find()
            .order_by_desc(patient::Column::CreatedAt)
            .all(db)
            .await
            .map_err(|e| format!("Failed to fetch patients: {}", e))?;

        Ok(results
            .into_iter()
            .map(|model| Patient {
                id: Some(model.id),
                uuid: model.uuid,
                first_name: model.first_name,
                last_name: model.last_name,
                date_of_birth: model.date_of_birth,
                patient_id_number: model.patient_id_number,
                email: model.email,
                phone: model.phone,
                notes: model.notes,
                created_at: model.created_at,
                updated_at: model.updated_at,
            })
            .collect())
    }

    // ------------------------------------------------------------------------
    // TEST OPERATIONS
    // ------------------------------------------------------------------------

    /// Insert a new test record
    pub async fn insert_test(db: &DatabaseConnection, test: &Test) -> Result<i64, String> {
        let test_model = test::ActiveModel {
            uuid: Set(test.uuid.clone()),
            patient_id: Set(test.patient_id),
            test_type: Set(test.test_type.clone()),
            device_id: Set(test.device_id.clone()),
            firmware_version: Set(test.firmware_version.clone()),
            detection_result: Set(test.detection_result.clone()),
            confidence: Set(test.confidence),
            raw_response: Set(test.raw_response.clone()),
            status: Set(test.status.clone()),
            error_message: Set(test.error_message.clone()),
            created_at: Set(test.created_at.clone()),
            updated_at: Set(test.updated_at.clone()),
            completed_at: Set(test.completed_at.clone()),
            ..Default::default()
        };

        let result = test::Entity::insert(test_model)
            .exec(db)
            .await
            .map_err(|e| format!("Failed to insert test: {}", e))?;

        Ok(result.last_insert_id)
    }

    /// Update an existing test record
    pub async fn update_test(db: &DatabaseConnection, test: &Test) -> Result<(), String> {
        let test_model = test::ActiveModel {
            id: Set(test.id.ok_or("Test has no ID")?),
            uuid: Set(test.uuid.clone()),
            patient_id: Set(test.patient_id),
            test_type: Set(test.test_type.clone()),
            device_id: Set(test.device_id.clone()),
            firmware_version: Set(test.firmware_version.clone()),
            detection_result: Set(test.detection_result.clone()),
            confidence: Set(test.confidence),
            raw_response: Set(test.raw_response.clone()),
            status: Set(test.status.clone()),
            error_message: Set(test.error_message.clone()),
            created_at: Set(test.created_at.clone()),
            updated_at: Set(test.updated_at.clone()),
            completed_at: Set(test.completed_at.clone()),
        };

        test::Entity::update(test_model)
            .exec(db)
            .await
            .map_err(|e| format!("Failed to update test: {}", e))?;

        Ok(())
    }

    /// Get a test by UUID
    pub async fn get_test_by_uuid(
        db: &DatabaseConnection,
        uuid: &str,
    ) -> Result<Option<Test>, String> {
        let result = test::Entity::find()
            .filter(test::Column::Uuid.eq(uuid))
            .one(db)
            .await
            .map_err(|e| format!("Failed to fetch test: {}", e))?;

        Ok(result.map(|model| Self::test_model_to_struct(model)))
    }

    /// Get all tests with patient information
    pub async fn get_all_tests_with_patients(
        db: &DatabaseConnection,
    ) -> Result<Vec<TestWithPatient>, String> {
        let results = test::Entity::find()
            .find_also_related(patient::Entity)
            .order_by_desc(test::Column::CreatedAt)
            .all(db)
            .await
            .map_err(|e| format!("Failed to fetch tests with patients: {}", e))?;

        let mut test_with_patients = Vec::new();

        for (test_model, patient_model) in results {
            if let Some(patient_model) = patient_model {
                let test = Self::test_model_to_struct(test_model);
                let patient = Self::patient_model_to_struct(patient_model);

                test_with_patients.push(TestWithPatient { test, patient });
            }
        }

        Ok(test_with_patients)
    }

    // ------------------------------------------------------------------------
    // HELPER FUNCTIONS
    // ------------------------------------------------------------------------

    fn patient_model_to_struct(model: patient::Model) -> Patient {
        Patient {
            id: Some(model.id),
            uuid: model.uuid,
            first_name: model.first_name,
            last_name: model.last_name,
            date_of_birth: model.date_of_birth,
            patient_id_number: model.patient_id_number,
            email: model.email,
            phone: model.phone,
            notes: model.notes,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }

    fn test_model_to_struct(model: test::Model) -> Test {
        Test {
            id: Some(model.id),
            uuid: model.uuid,
            patient_id: model.patient_id,
            test_type: model.test_type,
            device_id: model.device_id,
            firmware_version: model.firmware_version,
            detection_result: model.detection_result,
            confidence: model.confidence,
            raw_response: model.raw_response,
            status: model.status,
            error_message: model.error_message,
            created_at: model.created_at,
            updated_at: model.updated_at,
            completed_at: model.completed_at,
        }
    }
}
