// src/telemetry/error_rca.rs
// Automated Root Cause Analysis for Cargo & Agent Errors.

use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorRecord {
    pub timestamp: String,
    pub agent: String,
    pub error_code: String,
    pub raw_log: String,
    pub root_cause_category: String,
    pub auto_remediation_strategy: String,
}

pub struct ErrorTelemetryLogger;

impl ErrorTelemetryLogger {
    pub fn log_and_classify<P: AsRef<Path>>(
        log_path: P,
        agent: &str,
        raw_error: &str,
    ) -> ErrorRecord {
        let (code, category, remediation) = if raw_error.contains("E0382") {
            ("E0382", "BorrowChecker_MovedValue", "Introduce Arc::clone or refactor ownership boundaries.")
        } else if raw_error.contains("E0502") {
            ("E0502", "BorrowChecker_MutableAliasing", "Isolate mutable slice mutation before immutable reads.")
        } else if raw_error.contains("HTTP 403") || raw_error.contains("rate limit") {
            ("HTTP_RATE_LIMIT", "Network_Throttling", "Apply exponential backoff with jitter and rotate egress proxy.")
        } else {
            ("UNKNOWN_DEFECT", "General_Logic_Flaw", "Perform deep AST inspection and prompt recalibration.")
        };

        let record = ErrorRecord {
            timestamp: Local::now().to_rfc3339(),
            agent: agent.to_string(),
            error_code: code.to_string(),
            raw_log: raw_error.lines().take(5).collect::<Vec<_>>().join("\n"),
            root_cause_category: category.to_string(),
            auto_remediation_strategy: remediation.to_string(),
        };

        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) {
            if let Ok(json) = serde_json::to_string(&record) {
                let _ = writeln!(file, "{}", json);
            }
        }

        record
    }
}
