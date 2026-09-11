//! Validation report builder for theligi-validation.
use crate::{ValidationReport, ValidationResult};

/// Aggregate individual `ValidationResult`s into a `ValidationReport`.
pub fn build_report(results: Vec<ValidationResult>) -> ValidationReport {
    let total = results.len();
    let passed = results.iter().filter(|r| r.passed).count();
    let failed = total - passed;
    let all_violations: Vec<_> = results.iter().flat_map(|r| r.violations.clone()).collect();

    ValidationReport {
        total_series: total,
        passed,
        failed,
        results,
        all_violations,
    }
}
