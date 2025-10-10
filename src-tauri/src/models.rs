use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Type alias for the database pool state
pub type DbState = Mutex<SqlitePool>;

// ============================================================================
// PATIENT MODEL
// ============================================================================

/// Represents a patient record in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<NaiveDate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_id_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Patient {
    /// Create a new patient record
    pub fn new(
        first_name: String,
        last_name: String,
        date_of_birth: Option<NaiveDate>,
        patient_id_number: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        notes: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            uuid: Uuid::new_v4(),
            first_name,
            last_name,
            date_of_birth,
            patient_id_number,
            email,
            phone,
            notes,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get full name
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    /// Update the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

// ============================================================================
// ENUMS
// ============================================================================

/// Status of a test
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestStatus {
    Pending,
    InProgress,
    Completed,
    Error,
}

impl TestStatus {
    /// Convert to string for database storage
    pub fn as_str(&self) -> &str {
        match self {
            TestStatus::Pending => "pending",
            TestStatus::InProgress => "in_progress",
            TestStatus::Completed => "completed",
            TestStatus::Error => "error",
        }
    }

    /// Parse from string (from database)
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "pending" => Ok(TestStatus::Pending),
            "in_progress" => Ok(TestStatus::InProgress),
            "completed" => Ok(TestStatus::Completed),
            "error" => Ok(TestStatus::Error),
            _ => Err(format!("Invalid test status: {}", s)),
        }
    }
}

impl std::fmt::Display for TestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Type of test being performed
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TestType {
    Covid19,
    InfluenzaAB,
    StrepA,
    Rsv,
    Malaria,
    Hiv,
    Hepatitis,
    Other(String),
}

impl TestType {
    pub fn as_str(&self) -> &str {
        match self {
            TestType::Covid19 => "covid-19",
            TestType::InfluenzaAB => "influenza-ab",
            TestType::StrepA => "strep-a",
            TestType::Rsv => "rsv",
            TestType::Malaria => "malaria",
            TestType::Hiv => "hiv",
            TestType::Hepatitis => "hepatitis",
            TestType::Other(_) => "other",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "covid-19" => TestType::Covid19,
            "influenza-ab" => TestType::InfluenzaAB,
            "strep-a" => TestType::StrepA,
            "rsv" => TestType::Rsv,
            "malaria" => TestType::Malaria,
            "hiv" => TestType::Hiv,
            "hepatitis" => TestType::Hepatitis,
            other => TestType::Other(other.to_string()),
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            TestType::Covid19 => "COVID-19".to_string(),
            TestType::InfluenzaAB => "Influenza A/B".to_string(),
            TestType::StrepA => "Strep A".to_string(),
            TestType::Rsv => "RSV".to_string(),
            TestType::Malaria => "Malaria".to_string(),
            TestType::Hiv => "HIV".to_string(),
            TestType::Hepatitis => "Hepatitis".to_string(),
            TestType::Other(name) => name.clone(),
        }
    }
}

impl std::fmt::Display for TestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Detection result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DetectionResult {
    Positive,
    Negative,
    Inconclusive,
}

impl DetectionResult {
    pub fn as_str(&self) -> &str {
        match self {
            DetectionResult::Positive => "positive",
            DetectionResult::Negative => "negative",
            DetectionResult::Inconclusive => "inconclusive",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "positive" => Ok(DetectionResult::Positive),
            "negative" => Ok(DetectionResult::Negative),
            "inconclusive" => Ok(DetectionResult::Inconclusive),
            _ => Err(format!("Invalid detection result: {}", s)),
        }
    }
}

impl std::fmt::Display for DetectionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// TEST MODEL
// ============================================================================

/// Represents a test record in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Test {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    pub uuid: Uuid,
    pub patient_id: i64,
    pub test_type: TestType,

    // Device information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,

    // Detection results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_result: Option<DetectionResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_response: Option<String>,

    // Status tracking
    pub status: TestStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
}

impl Test {
    /// Create a new test record with pending status
    pub fn new_pending(
        patient_id: i64,
        test_type: TestType,
        device_id: Option<String>,
        firmware_version: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            uuid: Uuid::new_v4(),
            patient_id,
            test_type,
            device_id,
            firmware_version,
            detection_result: None,
            confidence: None,
            raw_response: None,
            status: TestStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
            completed_at: None,
        }
    }

    /// Update the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    /// Mark the test as in progress
    pub fn mark_in_progress(&mut self) {
        self.status = TestStatus::InProgress;
        self.touch();
    }

    /// Mark the test as completed with results
    pub fn mark_completed(
        &mut self,
        detection_result: DetectionResult,
        confidence: f64,
        raw_response: String,
    ) {
        self.detection_result = Some(detection_result);
        self.confidence = Some(confidence);
        self.raw_response = Some(raw_response);
        self.status = TestStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.touch();
    }

    /// Mark the test as failed
    pub fn mark_error(&mut self, error_message: String) {
        self.status = TestStatus::Error;
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
