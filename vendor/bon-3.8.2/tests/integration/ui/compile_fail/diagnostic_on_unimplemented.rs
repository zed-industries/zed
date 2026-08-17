use bon::Builder;

fn main() {
    #[derive(Builder)]
    struct Example {
        x: u32,
        y: u32,

        #[builder(name = renamed)]
        z: u32,
    }

    // Test error message about missing members
    let _ = Example::builder().x(1).build();

    // Test error message about repeated setter calls
    let _ = Example::builder().y(1).y(2);
}
