use snafu::{prelude::*, Location};

mod basics {
    use super::*;

    #[derive(Debug, Snafu)]
    enum Error {
        #[snafu(display("Created at {location}"))]
        Usage {
            #[snafu(implicit)]
            location: Location,
        },
    }

    #[test]
    fn location_tracks_file_line_and_column() {
        let one = UsageSnafu.build();
        let two = UsageSnafu.build();

        let sep = std::path::MAIN_SEPARATOR;
        assert_eq!(
            one.to_string(),
            format!("Created at tests{sep}location.rs:17:30")
        );
        assert_eq!(
            two.to_string(),
            format!("Created at tests{sep}location.rs:18:30")
        );
    }
}

mod track_caller {
    use super::*;

    #[derive(Debug, Copy, Clone, Snafu)]
    struct InnerError {
        #[snafu(implicit)]
        location: Location,
    }

    #[derive(Debug, Snafu)]
    struct WrapNoUserFieldsError {
        source: InnerError,
        #[snafu(implicit)]
        location: Location,
    }

    #[derive(Debug, Snafu)]
    #[snafu(context(false))]
    struct WrapNoContext {
        source: InnerError,
        #[snafu(implicit)]
        location: Location,
    }

    #[derive(Debug, Snafu)]
    #[snafu(display("{message}"))]
    #[snafu(whatever)]
    pub struct MyWhatever {
        #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
        source: Option<Box<dyn std::error::Error>>,
        message: String,
        #[snafu(implicit)]
        location: Location,
    }

    #[test]
    fn track_caller_is_applied_on_build() {
        let base_line = line!();
        let inner = InnerSnafu.build();
        assert_eq!(
            inner.location.line,
            base_line + 1,
            "Actual location: {}",
            inner.location,
        );
    }

    #[test]
    fn track_caller_is_applied_on_fail() {
        let base_line = line!();
        let inner = InnerSnafu.fail::<()>().unwrap_err();
        assert_eq!(
            inner.location.line,
            base_line + 1,
            "Actual location: {}",
            inner.location,
        );
    }

    #[test]
    fn track_caller_is_applied_on_ensure() {
        let base_line = line!();
        fn x() -> Result<(), InnerError> {
            ensure!(false, InnerSnafu);
            Ok(())
        }
        let inner = x().unwrap_err();
        assert_eq!(
            inner.location.line,
            base_line + 2,
            "Actual location: {}",
            inner.location,
        );
    }

    #[test]
    fn track_caller_is_applied_on_whatever() {
        let base_line = line!();
        fn x() -> Result<(), MyWhatever> {
            whatever!("bang");
        }
        let inner = x().unwrap_err();
        assert_eq!(
            inner.location.line,
            base_line + 2,
            "Actual location: {}",
            inner.location,
        );
    }

    #[test]
    fn track_caller_is_applied_on_result_context() {
        let base_line = line!();
        let wrap_no_user_fields = InnerSnafu
            .fail::<()>()
            .context(WrapNoUserFieldsSnafu)
            .unwrap_err();
        assert_eq!(
            wrap_no_user_fields.location.line,
            base_line + 3,
            "Actual location: {}",
            wrap_no_user_fields.location,
        );
    }

    #[test]
    fn track_caller_is_applied_on_result_with_context() {
        let base_line = line!();
        let wrap_no_user_fields = InnerSnafu
            .fail::<()>()
            .with_context(|_| WrapNoUserFieldsSnafu)
            .unwrap_err();
        assert_eq!(
            wrap_no_user_fields.location.line,
            base_line + 3,
            "Actual location: {}",
            wrap_no_user_fields.location,
        );
    }

    #[test]
    fn track_caller_is_applied_without_context() {
        let base_line = line!();
        let wrap_no_context: WrapNoContext = InnerSnafu.build().into();
        assert_eq!(
            wrap_no_context.location.line,
            base_line + 1,
            "Actual location: {}",
            wrap_no_context.location,
        );
    }

    #[test]
    fn track_caller_is_applied_on_result_whatever_context() {
        let base_line = line!();
        let whatever: MyWhatever = InnerSnafu
            .fail::<()>()
            .whatever_context("bang")
            .unwrap_err();
        assert_eq!(
            whatever.location.line,
            base_line + 3,
            "Actual location: {}",
            whatever.location,
        );
    }

    #[test]
    fn track_caller_is_applied_on_result_with_whatever_context() {
        let base_line = line!();
        let whatever: MyWhatever = InnerSnafu
            .fail::<()>()
            .with_whatever_context(|_| "bang")
            .unwrap_err();
        assert_eq!(
            whatever.location.line,
            base_line + 3,
            "Actual location: {}",
            whatever.location,
        );
    }

    #[test]
    fn track_caller_is_applied_on_option_context() {
        let base_line = line!();
        let option_to_error_no_user_fields = None::<()>.context(InnerSnafu).unwrap_err();
        assert_eq!(
            option_to_error_no_user_fields.location.line,
            base_line + 1,
            "Actual location: {}",
            option_to_error_no_user_fields.location,
        );
    }

    #[test]
    fn track_caller_is_applied_on_option_with_context() {
        let base_line = line!();
        let option_to_error_no_user_fields = None::<()>.with_context(|| InnerSnafu).unwrap_err();
        assert_eq!(
            option_to_error_no_user_fields.location.line,
            base_line + 1,
            "Actual location: {}",
            option_to_error_no_user_fields.location,
        );
    }

    #[test]
    fn track_caller_is_applied_on_option_whatever_context() {
        let base_line = line!();
        let whatever: MyWhatever = None::<()>.whatever_context("bang").unwrap_err();
        assert_eq!(
            whatever.location.line,
            base_line + 1,
            "Actual location: {}",
            whatever.location,
        );
    }

    #[test]
    fn track_caller_is_applied_on_option_with_whatever_context() {
        let base_line = line!();
        let whatever: MyWhatever = None::<()>.with_whatever_context(|| "bang").unwrap_err();
        assert_eq!(
            whatever.location.line,
            base_line + 1,
            "Actual location: {}",
            whatever.location,
        );
    }
}
