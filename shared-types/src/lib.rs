use serde::{Deserialize, Serialize};

// ============================================================================
// PATIENT MODEL
// ============================================================================

/// Represents a patient record in the database
/// Uses string types for cross-platform compatibility (WASM/native)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub uuid: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_id_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[cfg(feature = "backend")]
impl Patient {
    /// Create a new patient record with string-based timestamps
    pub fn new(
        first_name: String,
        last_name: String,
        date_of_birth: Option<String>,
        patient_id_number: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        notes: Option<String>,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: None,
            uuid: uuid::Uuid::new_v4().to_string(),
            first_name,
            last_name,
            date_of_birth,
            patient_id_number,
            email,
            phone,
            notes,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    /// Get full name (useful for display purposes)
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    /// Update the updated_at timestamp (useful for manual updates)
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

// ============================================================================
// TEST MODEL
// ============================================================================

/// Test types supported by the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TestType {
    #[serde(rename = "infectious_disease")]
    InfectiousDisease,
    #[serde(rename = "covid19")]
    Covid19,
    #[serde(rename = "flu")]
    Flu,
    #[serde(rename = "strep")]
    Strep,
}

impl TestType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TestType::InfectiousDisease => "infectious_disease",
            TestType::Covid19 => "covid19",
            TestType::Flu => "flu",
            TestType::Strep => "strep",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "infectious_disease" => Ok(TestType::InfectiousDisease),
            "covid19" => Ok(TestType::Covid19),
            "flu" => Ok(TestType::Flu),
            "strep" => Ok(TestType::Strep),
            _ => Err(format!("Unknown test type: {}", s)),
        }
    }
}

/// Test status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TestStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "cancelled")]
    Cancelled,
}

impl TestStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TestStatus::Pending => "pending",
            TestStatus::InProgress => "in_progress",
            TestStatus::Completed => "completed",
            TestStatus::Error => "error",
            TestStatus::Cancelled => "cancelled",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "pending" => Ok(TestStatus::Pending),
            "in_progress" => Ok(TestStatus::InProgress),
            "completed" => Ok(TestStatus::Completed),
            "error" => Ok(TestStatus::Error),
            "cancelled" => Ok(TestStatus::Cancelled),
            _ => Err(format!("Unknown test status: {}", s)),
        }
    }
}

/// Detection result enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DetectionResult {
    #[serde(rename = "positive")]
    Positive,
    #[serde(rename = "negative")]
    Negative,
    #[serde(rename = "inconclusive")]
    Inconclusive,
}

impl DetectionResult {
    pub fn as_str(&self) -> &'static str {
        match self {
            DetectionResult::Positive => "positive",
            DetectionResult::Negative => "negative",
            DetectionResult::Inconclusive => "inconclusive",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "positive" => Ok(DetectionResult::Positive),
            "negative" => Ok(DetectionResult::Negative),
            "inconclusive" => Ok(DetectionResult::Inconclusive),
            _ => Err(format!("Unknown detection result: {}", s)),
        }
    }
}

/// Represents a test record in the database
/// Uses string types for cross-platform compatibility (WASM/native)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Test {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub uuid: String,
    pub patient_id: i64,
    pub test_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_response: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

#[cfg(feature = "backend")]
impl Test {
    /// Create a new test record with pending status
    pub fn new_pending(
        patient_id: i64,
        test_type: String,
        device_id: Option<String>,
        firmware_version: Option<String>,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: None,
            uuid: uuid::Uuid::new_v4().to_string(),
            patient_id,
            test_type,
            device_id,
            firmware_version,
            detection_result: None,
            confidence: None,
            raw_response: None,
            status: "pending".to_string(),
            error_message: None,
            created_at: now.clone(),
            updated_at: now,
            completed_at: None,
        }
    }

    /// Update the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Mark the test as in progress
    pub fn mark_in_progress(&mut self) {
        self.status = "in_progress".to_string();
        self.touch();
    }

    /// Mark the test as completed with results
    pub fn mark_completed(
        &mut self,
        detection_result: String,
        confidence: f64,
        raw_response: String,
    ) {
        self.detection_result = Some(detection_result);
        self.confidence = Some(confidence);
        self.raw_response = Some(raw_response);
        self.status = "completed".to_string();
        self.completed_at = Some(chrono::Utc::now().to_rfc3339());
        self.touch();
    }

    /// Mark the test as failed
    pub fn mark_error(&mut self, error_message: String) {
        self.status = "error".to_string();
        self.error_message = Some(error_message);
        self.touch();
    }
}

// ============================================================================
// COMBINED MODEL FOR QUERIES
// ============================================================================

/// Represents a test with its associated patient information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestWithPatient {
    pub test: Test,
    pub patient: Patient,
}
