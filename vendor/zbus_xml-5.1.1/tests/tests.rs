use quick_xml::de::DeError;
use std::error::Error;

use zbus_xml::{ArgDirection, Node};

#[test]
fn serde() -> Result<(), Box<dyn Error>> {
    let example = include_str!("data/sample_object0.xml");
    let node_r = Node::from_reader(example.as_bytes())?;
    let node = Node::try_from(example)?;
    assert_eq!(node, node_r);
    assert_eq!(node.interfaces().len(), 1);
    assert_eq!(node.interfaces()[0].methods().len(), 3);
    assert_eq!(
        node.interfaces()[0].methods()[0].args()[0]
            .direction()
            .unwrap(),
        ArgDirection::In
    );
    assert_eq!(node.nodes().len(), 4);

    let node_str: Node<'_> = example.try_into()?;
    assert_eq!(node_str.interfaces().len(), 1);
    assert_eq!(node_str.nodes().len(), 4);

    let mut writer = Vec::with_capacity(128);
    node.to_writer(&mut writer).unwrap();
    Ok(())
}

#[test]
fn invalid_arg_type() {
    let input = include_str!("data/invalid_arg_type.xml");
    assert!(matches!(
        Node::try_from(input),
        Err(zbus_xml::Error::QuickXml(DeError::Custom(_)))
    ));
}
