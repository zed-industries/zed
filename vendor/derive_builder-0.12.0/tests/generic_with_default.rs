#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate pretty_assertions;

/// Struct taken from `@shockham/caper` to make sure we emit the correct
/// code for struct-level defaults in tandem with generics.
#[derive(Builder, Clone, PartialEq)]
#[builder(default)]
pub struct RenderItem<T: Default> {
    /// The vertices representing this items mesh
    pub vertices: Vec<()>,
    /// Whether the item is active/should be rendered
    pub active: bool,
    /// The name of the RenderItem for lookup
    pub name: String,
    /// Tag Type for grouping similar items
    pub tag: T,
}

impl<T: Default> Default for RenderItem<T> {
    fn default() -> Self {
        RenderItem {
            vertices: Default::default(),
            active: true,
            name: "ri".into(),
            tag: Default::default(),
        }
    }
}

#[test]
fn create_with_string() {
    let ri: RenderItem<String> = RenderItemBuilder::default().build().unwrap();
    assert_eq!(ri.tag, "");
    assert_eq!(ri.name, "ri");
    assert!(ri.active);
}
