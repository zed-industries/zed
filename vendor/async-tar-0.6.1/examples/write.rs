#[cfg(feature = "runtime-async-std")]
use async_std::fs::File;
use async_tar::Builder;
#[cfg(feature = "runtime-tokio")]
use tokio::fs::File;

async fn inner_main() {
    let file = File::create("foo.tar").await.unwrap();
    let mut a = Builder::new(file);

    a.append_path("README.md").await.unwrap();
    a.append_file("lib.rs", &mut File::open("src/lib.rs").await.unwrap())
        .await
        .unwrap();
}

#[cfg(feature = "runtime-async-std")]
fn main() {
    async_std::task::block_on(inner_main());
}

#[cfg(feature = "runtime-tokio")]
fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(inner_main());
}
