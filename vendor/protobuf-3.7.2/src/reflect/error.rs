#[derive(Debug, thiserror::Error)]
pub(crate) enum ReflectError {
    #[error("Message `{}` not found in files: {}", .0, .1)]
    MessageNotFoundInFiles(String, String),
    #[error("Dependency `{}` of `{}` not found; all deps: {}", .0, .1, .2)]
    DependencyNotFound(String, String, String),
    #[error("Non-unique dependencies given: {}", .0)]
    NonUniqueDependencies(String),
    #[error("Non-unique field name: `{0}`")]
    NonUniqueFieldName(String),
    #[error("Non-unique file descriptor: `{0}`")]
    NonUniqueFileDescriptor(String),
    #[error("Cycle in provided file descriptors")]
    CycleInFileDescriptors,
    #[error("Map entry message name must end with `Entry`")]
    MapEntryNameMustEndWithEntry,
    #[error("Map entry message must have no extensions, nested messages or enums")]
    MapEntryMustHaveNo,
    #[error(
        "Map entry message must have two optional fields, \
        numbered 1 and 2 and named `key` and `value`"
    )]
    MapEntryIncorrectFields,
    #[error("Could not parse default value for field {0}")]
    CouldNotParseDefaultValueForField(String),
}
