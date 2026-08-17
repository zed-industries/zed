#[cxx::bridge]
mod ffi {
    struct Node0 {
        i: i32,
    }

    struct Node1 {
        node2: Node2,
        vec: Vec<Node3>,
    }

    struct Node2 {
        node4: Node4,
    }

    struct Node3 {
        node1: Node1,
    }

    struct Node4 {
        node0: Node0,
        node5: Node5,
    }

    struct Node5 {
        node2: Node2,
    }

    struct Node6 {
        node2: Node2,
    }
}

fn main() {}
