use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

use super::models::QualityMetrics;

pub struct LocalQualityProvider {
    pub path: PathBuf,
}

impl LocalQualityProvider {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn fetch_metrics(&self) -> Result<QualityMetrics> {
        let lint_warnings = self.count_clippy_warnings();
        let (lint_errors, _) = self.count_clippy_errors();
        let test_coverage = self.estimate_test_coverage();
        let security_issues = 0; // Would need cargo-audit

        Ok(QualityMetrics {
            test_coverage,
            lint_warnings,
            lint_errors,
            security_issues,
        })
    }

    fn count_clippy_warnings(&self) -> usize {
        let output = Command::new("cargo")
            .args(["clippy", "--message-format=json", "-q"])
            .current_dir(&self.path)
            .env("PATH", Self::cargo_path())
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                stdout
                    .lines()
                    .filter(|line| {
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
                            v["reason"].as_str() == Some("compiler-message")
                                && v["message"]["level"].as_str() == Some("warning")
                        } else {
                            false
                        }
                    })
                    .count()
            }
            Err(_) => 0,
        }
    }

    fn count_clippy_errors(&self) -> (usize, usize) {
        let output = Command::new("cargo")
            .args(["clippy", "--message-format=json", "-q"])
            .current_dir(&self.path)
            .env("PATH", Self::cargo_path())
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let errors = stdout
                    .lines()
                    .filter(|line| {
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
                            v["reason"].as_str() == Some("compiler-message")
                                && v["message"]["level"].as_str() == Some("error")
                        } else {
                            false
                        }
                    })
                    .count();
                (errors, 0)
            }
            Err(_) => (0, 0),
        }
    }

    fn estimate_test_coverage(&self) -> f64 {
        // Run cargo test and count pass/fail
        let output = Command::new("cargo")
            .args(["test", "--", "--format=terse"])
            .current_dir(&self.path)
            .env("PATH", Self::cargo_path())
            .output();

        match output {
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                let mut total = 0u64;
                let mut passed = 0u64;

                for line in stderr.lines() {
                    if line.contains("test result:") {
                        // Parse: "test result: ok. X passed; Y failed; ..."
                        if let Some(p) = line.split("passed").next() {
                            if let Some(num) = p.split_whitespace().last() {
                                passed += num.parse::<u64>().unwrap_or(0);
                            }
                        }
                        if let Some(f) = line.split("failed").next() {
                            let failed: u64 = f
                                .split_whitespace()
                                .last()
                                .and_then(|n| n.parse().ok())
                                .unwrap_or(0);
                            total += passed + failed;
                        }
                    }
                }

                if total > 0 {
                    (passed as f64 / total as f64) * 100.0
                } else {
                    0.0
                }
            }
            Err(_) => 0.0,
        }
    }

    fn cargo_path() -> String {
        let home = std::env::var("HOME").unwrap_or_default();
        format!("{}/.cargo/bin:/usr/local/bin:/usr/bin:/bin", home)
    }
}
