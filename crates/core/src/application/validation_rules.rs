//! Validation rules for CyberWiki.
//!
//! Encapsulates structural checks, blacklist enforcement, and basic
//! editorial constraints.

use crate::domain::term_entity::Term;

/// Result type for validation operations.
pub enum ValidationOutcome {
    Accepted,
    Rejected { reason: String },
}

/// Stateless validation utilities.
pub struct ValidationRules;

impl ValidationRules {
    pub fn validate_term(term: &Term) -> ValidationOutcome {
        if term.name.trim().is_empty() {
            return ValidationOutcome::Rejected {
                reason: "Term name must not be empty.".to_string(),
            };
        }

        if term.short_definition.len() < 8 {
            return ValidationOutcome::Rejected {
                reason: "Short definition is too short for a technical term."
                    .to_string(),
            };
        }

        ValidationOutcome::Accepted
    }
}
