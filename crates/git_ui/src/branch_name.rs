pub(super) fn normalize_branch_name(query: &str) -> String {
    query.trim().replace(' ', "-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_branch_name() {
        assert_eq!(normalize_branch_name(" branch-name "), "branch-name");
        assert_eq!(normalize_branch_name("branch name"), "branch-name");
        assert_eq!(normalize_branch_name("  branch  name  "), "branch--name");
    }
}
