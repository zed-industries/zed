/// Represents a parsed insight extracted from an EXPLAIN plan output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanInsight {
    pub severity: InsightSeverity,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InsightSeverity {
    /// Potential performance issue that should be investigated.
    Warning,
    /// Informational observation about the plan.
    Info,
}

/// Parses the raw text output of an EXPLAIN plan and extracts actionable insights.
///
/// Supports output from SQLite (EXPLAIN QUERY PLAN), PostgreSQL (EXPLAIN), and
/// MySQL (EXPLAIN) by looking for known patterns in the plan text.
pub fn analyze_plan(plan_text: &str) -> Vec<PlanInsight> {
    let mut insights = Vec::new();

    let upper = plan_text.to_uppercase();

    // Sequential / full scans
    if upper.contains("SEQ SCAN") || upper.contains("FULL SCAN") || upper.contains("SCAN TABLE") {
        insights.push(PlanInsight {
            severity: InsightSeverity::Warning,
            message: "Full table scan detected. Consider adding an index on the filtered or \
                joined columns to avoid reading the entire table."
                .to_string(),
        });
    }

    // Nested loop without index
    if upper.contains("NESTED LOOP") && !upper.contains("INDEX") {
        insights.push(PlanInsight {
            severity: InsightSeverity::Warning,
            message: "Nested loop join without index access detected. An index on the join \
                column of the inner table would significantly improve performance."
                .to_string(),
        });
    }

    // Hash join (not a warning, but informational for large tables)
    if upper.contains("HASH JOIN") || upper.contains("HASH INNER JOIN") {
        insights.push(PlanInsight {
            severity: InsightSeverity::Info,
            message: "Hash join detected. This is efficient for large tables but requires \
                sufficient working memory (work_mem in PostgreSQL)."
                .to_string(),
        });
    }

    // Sort without index
    if upper.contains("SORT") && !upper.contains("INDEX SCAN") && !upper.contains("INDEX ONLY") {
        insights.push(PlanInsight {
            severity: InsightSeverity::Warning,
            message: "Sort operation without index support detected. An index on the ORDER BY \
                column(s) can eliminate the sort step."
                .to_string(),
        });
    }

    // Bitmap heap scan (PostgreSQL) — generally good but worth noting
    if upper.contains("BITMAP HEAP SCAN") {
        insights.push(PlanInsight {
            severity: InsightSeverity::Info,
            message: "Bitmap heap scan used. This is a two-phase index strategy effective for \
                moderate selectivity. For highly selective queries, an index scan may be faster."
                .to_string(),
        });
    }

    // Temporary files (spill to disk)
    if upper.contains("SORT METHOD: EXTERNAL") || upper.contains("BATCHES:") {
        insights.push(PlanInsight {
            severity: InsightSeverity::Warning,
            message: "Query spills to disk. Increase work_mem (PostgreSQL) or sort_buffer_size \
                (MySQL) to keep sorting in memory."
                .to_string(),
        });
    }

    // Index usage (positive signal)
    if upper.contains("INDEX SCAN")
        || upper.contains("INDEX ONLY SCAN")
        || upper.contains("USING INDEX")
    {
        insights.push(PlanInsight {
            severity: InsightSeverity::Info,
            message: "Index scan detected — the query is using an index efficiently.".to_string(),
        });
    }

    insights
}

/// Formats a list of plan insights into a human-readable markdown summary.
pub fn format_insights(insights: &[PlanInsight]) -> String {
    if insights.is_empty() {
        return "No notable performance issues detected in the execution plan.".to_string();
    }

    let mut output = String::from("## Execution Plan Analysis\n\n");
    for insight in insights {
        let prefix = match insight.severity {
            InsightSeverity::Warning => "⚠️ **Warning**",
            InsightSeverity::Info => "ℹ️ **Info**",
        };
        output.push_str(&format!("- {prefix}: {}\n", insight.message));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_seq_scan() {
        let plan = "Seq Scan on users (cost=0.00..100.00 rows=1000)";
        let insights = analyze_plan(plan);
        assert!(insights
            .iter()
            .any(|i| i.severity == InsightSeverity::Warning && i.message.contains("Full table scan")));
    }

    #[test]
    fn test_detect_index_scan() {
        let plan = "Index Scan using users_pkey on users (cost=0.29..8.31 rows=1)";
        let insights = analyze_plan(plan);
        assert!(insights
            .iter()
            .any(|i| i.severity == InsightSeverity::Info && i.message.contains("Index scan")));
    }

    #[test]
    fn test_detect_sort_warning() {
        let plan = "Sort (cost=500.00..510.00)\n  Sort Key: created_at\n  Seq Scan on events";
        let insights = analyze_plan(plan);
        let has_sort_warning = insights
            .iter()
            .any(|i| i.severity == InsightSeverity::Warning && i.message.contains("Sort"));
        assert!(has_sort_warning);
    }

    #[test]
    fn test_detect_hash_join_info() {
        let plan = "Hash Join (cost=100.00..500.00)\n  Hash Cond: (a.id = b.id)";
        let insights = analyze_plan(plan);
        assert!(insights
            .iter()
            .any(|i| i.severity == InsightSeverity::Info && i.message.contains("Hash join")));
    }

    #[test]
    fn test_clean_plan_no_warnings() {
        let plan = "Index Only Scan using users_pkey on users";
        let insights = analyze_plan(plan);
        assert!(!insights
            .iter()
            .any(|i| i.severity == InsightSeverity::Warning));
    }

    #[test]
    fn test_format_insights_empty() {
        let text = format_insights(&[]);
        assert!(text.contains("No notable performance issues"));
    }

    #[test]
    fn test_format_insights_with_warning() {
        let insights = vec![PlanInsight {
            severity: InsightSeverity::Warning,
            message: "Full table scan detected.".to_string(),
        }];
        let text = format_insights(&insights);
        assert!(text.contains("Warning"));
        assert!(text.contains("Full table scan"));
    }
}
