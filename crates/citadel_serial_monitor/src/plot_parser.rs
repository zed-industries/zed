/// One value parsed out of a received serial line, for the Serial Plotter.
#[derive(Debug, Clone, PartialEq)]
pub struct PlotPoint {
    pub label: String,
    pub value: f32,
}

/// Parses one received line into zero or more labeled numeric points,
/// following the Arduino Plotter convention: tokens are separated by `,`,
/// tab, or space. A token shaped `<label>:<number>` names its series
/// explicitly; a bare `<number>` token is assigned to an auto-named series
/// (`value1`, `value2`, ... by position among the bare tokens in this
/// line). Tokens that parse as neither are silently skipped -- ordinary
/// log lines interleaved with numeric telemetry are expected, not an
/// error.
pub fn parse_plot_line(line: &str) -> Vec<PlotPoint> {
    let mut points = Vec::new();
    let mut bare_index = 0;
    for token in line.split(|c: char| c == ',' || c == '\t' || c == ' ') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        if let Some((label, value)) = token.split_once(':') {
            if let Ok(value) = value.trim().parse::<f32>() {
                points.push(PlotPoint {
                    label: label.trim().to_string(),
                    value,
                });
                continue;
            }
        }
        if let Ok(value) = token.parse::<f32>() {
            bare_index += 1;
            points.push(PlotPoint {
                label: format!("value{bare_index}"),
                value,
            });
        }
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_labeled_values() {
        let points = parse_plot_line("temp:23.5,humidity:60");
        assert_eq!(
            points,
            vec![
                PlotPoint { label: "temp".to_string(), value: 23.5 },
                PlotPoint { label: "humidity".to_string(), value: 60.0 },
            ]
        );
    }

    #[test]
    fn test_parse_bare_values_auto_named() {
        let points = parse_plot_line("23.5,60");
        assert_eq!(
            points,
            vec![
                PlotPoint { label: "value1".to_string(), value: 23.5 },
                PlotPoint { label: "value2".to_string(), value: 60.0 },
            ]
        );
    }

    #[test]
    fn test_parse_mixed_labeled_and_bare() {
        let points = parse_plot_line("label:1.0, 2.0");
        assert_eq!(
            points,
            vec![
                PlotPoint { label: "label".to_string(), value: 1.0 },
                PlotPoint { label: "value1".to_string(), value: 2.0 },
            ]
        );
    }

    #[test]
    fn test_parse_skips_non_numeric_tokens() {
        let points = parse_plot_line("hello world 42");
        assert_eq!(points, vec![PlotPoint { label: "value1".to_string(), value: 42.0 }]);
    }

    #[test]
    fn test_parse_empty_line() {
        assert_eq!(parse_plot_line(""), Vec::new());
    }

    #[test]
    fn test_parse_labeled_token_with_non_numeric_value_is_skipped() {
        assert_eq!(parse_plot_line("label:notanumber"), Vec::new());
    }

    #[test]
    fn test_parse_tab_separated() {
        let points = parse_plot_line("1.0\t2.0");
        assert_eq!(
            points,
            vec![
                PlotPoint { label: "value1".to_string(), value: 1.0 },
                PlotPoint { label: "value2".to_string(), value: 2.0 },
            ]
        );
    }
}
