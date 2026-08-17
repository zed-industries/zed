use base64::{engine::general_purpose::STANDARD as B64, Engine};
use crypto_box::{self, aead::OsRng, PublicKey};
use octocrab::{models::repos::secrets::CreateRepositorySecret, Octocrab};
use std::convert::TryInto;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;
    let repo = octocrab.repos("owner", "repo");
    let secrets = repo.secrets();

    let public_key = secrets.get_public_key().await?;

    let crypto_pk = {
        let pk_bytes = B64.decode(public_key.key).unwrap();
        let pk_array: [u8; crypto_box::KEY_SIZE] = pk_bytes.try_into().unwrap();
        PublicKey::from(pk_array)
    };

    let encrypted_value = crypto_box::seal(&mut OsRng, &crypto_pk, b"Very secret value").unwrap();

    let result = secrets
        .create_or_update_secret(
            "TEST_SECRET_RS",
            &CreateRepositorySecret {
                encrypted_value: &B64.encode(encrypted_value),
                key_id: &public_key.key_id,
            },
        )
        .await?;

    println!("{result:?}");

    Ok(())
}
