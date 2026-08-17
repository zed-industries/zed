use taffy::prelude::*;

fn main() -> Result<(), taffy::TaffyError> {
    let mut taffy: TaffyTree<()> = TaffyTree::new();

    // left
    let child_t1 = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_length(5.0), height: Dimension::from_length(5.0) },
        ..Default::default()
    })?;

    let div1 = taffy.new_with_children(
        Style {
            size: Size { width: Dimension::from_percent(0.5), height: Dimension::from_percent(1.0) },
            // justify_content: JustifyContent::Center,
            ..Default::default()
        },
        &[child_t1],
    )?;

    // right
    let child_t2 = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_length(5.0), height: Dimension::from_length(5.0) },
        ..Default::default()
    })?;

    let div2 = taffy.new_with_children(
        Style {
            size: Size { width: Dimension::from_percent(0.5), height: Dimension::from_percent(1.0) },
            // justify_content: JustifyContent::Center,
            ..Default::default()
        },
        &[child_t2],
    )?;

    let container = taffy.new_with_children(
        Style {
            size: Size { width: Dimension::from_percent(1.0), height: Dimension::from_percent(1.0) },
            ..Default::default()
        },
        &[div1, div2],
    )?;

    taffy.compute_layout(
        container,
        Size { height: AvailableSpace::Definite(100.0), width: AvailableSpace::Definite(100.0) },
    )?;

    println!("node: {:#?}", taffy.layout(container)?);

    println!("div1: {:#?}", taffy.layout(div1)?);
    println!("div2: {:#?}", taffy.layout(div2)?);

    println!("child1: {:#?}", taffy.layout(child_t1)?);
    println!("child2: {:#?}", taffy.layout(child_t2)?);

    Ok(())
}
