use async_std::task;

fn main() -> Result<(), surf::Error> {
    task::block_on(async {
        let url = "https://www.rust-lang.org";
        let mut response = surf::get(url).send().await?;
        let body = response.body_string().await?;

        dbg!(url);
        dbg!(response.status());
        dbg!(response.version());
        dbg!(response.header_names());
        dbg!(response.header_values());
        dbg!(body.len());

        Ok(())
    })
}
