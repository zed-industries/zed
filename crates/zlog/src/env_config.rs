use anyhow::{Result, anyhow};

pub struct EnvFilter {
    pub level_global: Option<log::LevelFilter>,
    pub directive_names: Vec<String>,
    pub directive_levels: Vec<log::LevelFilter>,
}

pub fn parse(filter: &str) -> Result<EnvFilter> {
    let mut max_level = None;
    let mut directive_names = Vec::new();
    let mut directive_levels = Vec::new();

    for directive in filter.split(',') {
        match directive.split_once('=') {
            Some((name, level)) => {
                if level.contains('=') {
                    return Err(anyhow!("Invalid directive: {}", directive));
                }
                let level = parse_level(level.trim())?;
                directive_names.push(name.to_string());
                directive_levels.push(level);
            }
            None => {
                let Ok(level) = parse_level(directive.trim()) else {
                    directive_names.push(directive.trim().to_string());
                    directive_levels.push(log::LevelFilter::max() /* Enable all levels */);
                    continue;
                };
                if max_level.is_some() {
                    return Err(anyhow!("Cannot set multiple max levels"));
                }
                max_level.replace(level);
            }
        };
    }

    Ok(EnvFilter {
        level_global: max_level,
        directive_names,
        directive_levels,
    })
}

fn parse_level(level: &str) -> Result<log::LevelFilter> {
    if level.eq_ignore_ascii_case("TRACE") {
        return Ok(log::LevelFilter::Trace);
    }
    if level.eq_ignore_ascii_case("DEBUG") {
        return Ok(log::LevelFilter::Debug);
    }
    if level.eq_ignore_ascii_case("INFO") {
        return Ok(log::LevelFilter::Info);
    }
    if level.eq_ignore_ascii_case("WARN") {
        return Ok(log::LevelFilter::Warn);
    }
    if level.eq_ignore_ascii_case("ERROR") {
        return Ok(log::LevelFilter::Error);
    }
    if level.eq_ignore_ascii_case("OFF") || level.eq_ignore_ascii_case("NONE") {
        return Ok(log::LevelFilter::Off);
    }
    Err(anyhow!("Invalid level: {}", level))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_level() {
        let input = "info";
        let filter = parse(input).unwrap();

        assert_eq!(filter.level_global.unwrap(), log::LevelFilter::Info);
        assert!(filter.directive_names.is_empty());
        assert!(filter.directive_levels.is_empty());
    }

    #[test]
    fn directive_level() {
        let input = "my_module=debug";
        let filter = parse(input).unwrap();

        assert_eq!(filter.level_global, None);
        assert_eq!(filter.directive_names, vec!["my_module".to_string()]);
        assert_eq!(filter.directive_levels, vec![log::LevelFilter::Debug]);
    }

    #[test]
    fn global_level_and_directive_level() {
        let input = "info,my_module=debug";
        let filter = parse(input).unwrap();

        assert_eq!(filter.level_global.unwrap(), log::LevelFilter::Info);
        assert_eq!(filter.directive_names, vec!["my_module".to_string()]);
        assert_eq!(filter.directive_levels, vec![log::LevelFilter::Debug]);
    }

    #[test]
    fn global_level_and_bare_module() {
        let input = "info,my_module";
        let filter = parse(input).unwrap();

        assert_eq!(filter.level_global.unwrap(), log::LevelFilter::Info);
        assert_eq!(filter.directive_names, vec!["my_module".to_string()]);
        assert_eq!(filter.directive_levels, vec![log::LevelFilter::max()]);
    }

    #[test]
    fn err_when_multiple_max_levels() {
        let input = "info,warn";
        let result = parse(input);

        assert!(result.is_err());
    }

    #[test]
    fn err_when_invalid_level() {
        let input = "my_module=foobar";
        let result = parse(input);

        assert!(result.is_err());
    }
}
