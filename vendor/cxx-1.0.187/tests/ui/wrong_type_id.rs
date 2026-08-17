#[cxx::bridge(namespace = "folly")]
mod here {
    extern "C++" {
        type StringPiece;
    }
}

#[cxx::bridge(namespace = "folly")]
mod there {
    extern "C++" {
        type ByteRange = crate::here::StringPiece;
    }
}

fn main() {}
