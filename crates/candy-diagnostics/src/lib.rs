use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub file: String,
    pub start_line: u32,
    pub start_col: u32,
    pub end_line: u32,
    pub end_col: u32,
}

impl Span {
    /// Unknown span for fallback paths. Avoid in real parser/typecheck outputs.
    pub fn unknown(file: impl Into<String>) -> Self {
        Self {
            file: file.into(),
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
        }
    }

    pub fn single_point(file: impl Into<String>, line: u32, col: u32) -> Self {
        let file = file.into();
        Self {
            file,
            start_line: line,
            start_col: col,
            end_line: line,
            end_col: col,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fix {
    /// Suggested replacement as a minimal patch hint (not guaranteed to apply).
    pub replace: String,
    pub with: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Stable machine code, e.g. "parse-unexpected-token", "type-mismatch"
    pub code: String,
    pub severity: Severity,
    pub message: String,
    pub span: Span,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix: Option<Fix>,
}

impl Diagnostic {
    pub fn error(code: impl Into<String>, message: impl Into<String>, span: Span) -> Self {
        Self {
            code: code.into(),
            severity: Severity::Error,
            message: message.into(),
            span,
            fix: None,
        }
    }

    pub fn warning(code: impl Into<String>, message: impl Into<String>, span: Span) -> Self {
        Self {
            code: code.into(),
            severity: Severity::Warning,
            message: message.into(),
            span,
            fix: None,
        }
    }

    pub fn with_fix(mut self, replace: impl Into<String>, with: impl Into<String>) -> Self {
        self.fix = Some(Fix {
            replace: replace.into(),
            with: with.into(),
        });
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticReport {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn push(&mut self, d: Diagnostic) {
        self.diagnostics.push(d);
    }

    pub fn is_ok(&self) -> bool {
        !self
            .diagnostics
            .iter()
            .any(|d| matches!(d.severity, Severity::Error))
    }

    /// Agent mode: JSON only, stable schema.
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("diagnostic JSON serialization must not fail")
    }
}

impl Default for DiagnosticReport {
    fn default() -> Self {
        Self::new()
    }
}
