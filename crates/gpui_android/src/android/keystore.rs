//! Credentials backed by `AndroidKeyStore` + on-disk encrypted blobs.
//!
//! Approach (deliberately minimal, no `androidx.security` dependency):
//!
//! 1. Generate (or fetch) an `AndroidKeyStore`-backed AES-256-GCM key whose
//!    alias is derived from the credential URL. The key never leaves the
//!    TEE, so the on-disk blob is useless without the device.
//! 2. Encrypt `{ username\0password }` with that key. AES-GCM emits an IV
//!    that we serialise alongside the ciphertext.
//! 3. Store the blob at `<filesDir>/zed-credentials/<sanitized-url>.bin`.
//!
//! All errors propagate up via `anyhow`; the `Platform` impl turns them into
//! `Task<Result<...>>` errors so the calling UI gets a real failure message.

use anyhow::{Context as _, Result, anyhow};
use android_activity::AndroidApp;
use jni::{
    Env, jni_sig, jni_str,
    objects::{JObject, JValue},
};

use super::jni_glue::{byte_array_to_vec, java_string_to_rust, new_byte_array, with_activity};

const ALIAS_PREFIX: &str = "zed.credentials.";
const STORAGE_DIR: &str = "zed-credentials";
const SEPARATOR: u8 = 0;

pub(crate) fn write(
    app: &AndroidApp,
    url: &str,
    username: &str,
    password: &[u8],
) -> Result<()> {
    let alias = make_alias(url);
    let storage_path = storage_path_for(app, url)?;
    if let Some(parent) = storage_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let mut plaintext = Vec::with_capacity(username.len() + 1 + password.len());
    plaintext.extend_from_slice(username.as_bytes());
    plaintext.push(SEPARATOR);
    plaintext.extend_from_slice(password);

    let (iv, ciphertext) = with_activity(app, |env, _| {
        ensure_key(env, &alias)?;
        encrypt(env, &alias, &plaintext)
    })?;

    let mut blob = Vec::with_capacity(4 + iv.len() + ciphertext.len());
    blob.extend_from_slice(&(iv.len() as u32).to_le_bytes());
    blob.extend_from_slice(&iv);
    blob.extend_from_slice(&ciphertext);
    std::fs::write(storage_path, blob).context("writing credential blob")?;
    Ok(())
}

pub(crate) fn read(app: &AndroidApp, url: &str) -> Result<Option<(String, Vec<u8>)>> {
    let alias = make_alias(url);
    let storage_path = storage_path_for(app, url)?;
    if !storage_path.exists() {
        return Ok(None);
    }
    let blob = std::fs::read(&storage_path).context("reading credential blob")?;
    if blob.len() < 4 {
        return Ok(None);
    }
    let iv_len = u32::from_le_bytes(blob[..4].try_into().unwrap()) as usize;
    if blob.len() < 4 + iv_len {
        return Ok(None);
    }
    let iv = blob[4..4 + iv_len].to_vec();
    let ciphertext = blob[4 + iv_len..].to_vec();

    let plaintext = with_activity(app, |env, _| decrypt(env, &alias, &iv, &ciphertext))?;
    let mut split = plaintext.splitn(2, |b| *b == SEPARATOR);
    let username = match split.next() {
        Some(u) => String::from_utf8_lossy(u).into_owned(),
        None => return Ok(None),
    };
    let password = split.next().map(|p| p.to_vec()).unwrap_or_default();
    Ok(Some((username, password)))
}

pub(crate) fn delete(app: &AndroidApp, url: &str) -> Result<()> {
    let alias = make_alias(url);
    let storage_path = storage_path_for(app, url)?;
    let _ = std::fs::remove_file(&storage_path);
    let _ = with_activity(app, |env, _| delete_key(env, &alias));
    Ok(())
}

fn make_alias(url: &str) -> String {
    let sanitized: String = url
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '.' { c } else { '_' })
        .collect();
    format!("{ALIAS_PREFIX}{sanitized}")
}

fn storage_path_for(app: &AndroidApp, url: &str) -> Result<std::path::PathBuf> {
    let files_dir = with_activity(app, |env, activity| {
        let dir = env
            .call_method(
                activity,
                jni_str!("getFilesDir"),
                jni_sig!(() -> "java.io.File"),
                &[],
            )
            .context("Context.getFilesDir")?
            .l()
            .context("getFilesDir returned non-object")?;
        let path = env
            .call_method(
                &dir,
                jni_str!("getAbsolutePath"),
                jni_sig!(() -> "java.lang.String"),
                &[],
            )
            .context("File.getAbsolutePath")?
            .l()
            .context("getAbsolutePath returned non-object")?;
        java_string_to_rust(env, path)
    })?;

    let mut path = std::path::PathBuf::from(files_dir);
    path.push(STORAGE_DIR);
    let alias = make_alias(url);
    path.push(format!("{}.bin", &alias[ALIAS_PREFIX.len()..]));
    Ok(path)
}

fn ensure_key<'local>(env: &mut Env<'local>, alias: &str) -> Result<()> {
    let keystore = open_keystore(env)?;
    let alias_jstr = env.new_string(alias).context("alloc alias")?;
    let exists = env
        .call_method(
            &keystore,
            jni_str!("containsAlias"),
            jni_sig!((alias: "java.lang.String") -> bool),
            &[JValue::Object(&alias_jstr)],
        )
        .context("KeyStore.containsAlias")?
        .z()
        .context("containsAlias returned non-bool")?;
    if exists {
        return Ok(());
    }
    generate_key(env, alias)
}

fn open_keystore<'local>(env: &mut Env<'local>) -> Result<JObject<'local>> {
    let provider = env
        .new_string("AndroidKeyStore")
        .context("alloc provider")?;
    let class = env
        .find_class(jni_str!("java/security/KeyStore"))
        .context("FindClass KeyStore")?;
    let store = env
        .call_static_method(
            &class,
            jni_str!("getInstance"),
            jni_sig!((provider: "java.lang.String") -> "java.security.KeyStore"),
            &[JValue::Object(&provider)],
        )
        .context("KeyStore.getInstance")?
        .l()
        .context("KeyStore.getInstance returned non-object")?;
    env.call_method(
        &store,
        jni_str!("load"),
        jni_sig!((param: "java.security.KeyStore$LoadStoreParameter") -> void),
        &[JValue::Object(&JObject::null())],
    )
    .context("KeyStore.load(null)")?;
    Ok(store)
}

fn generate_key<'local>(env: &mut Env<'local>, alias: &str) -> Result<()> {
    // KeyGenParameterSpec.Builder builder =
    //     new KeyGenParameterSpec.Builder(alias, ENCRYPT|DECRYPT);
    let alias_jstr = env.new_string(alias).context("alloc alias")?;
    let builder_class = env
        .find_class(jni_str!("android/security/keystore/KeyGenParameterSpec$Builder"))
        .context("FindClass KeyGenParameterSpec.Builder")?;
    // KeyProperties.PURPOSE_ENCRYPT (1) | KeyProperties.PURPOSE_DECRYPT (2) = 3
    let builder = env
        .new_object(
            &builder_class,
            jni_sig!((alias: "java.lang.String", purposes: jint) -> void),
            &[JValue::Object(&alias_jstr), JValue::Int(3)],
        )
        .context("new KeyGenParameterSpec.Builder")?;

    let block_modes = env
        .new_string("GCM")
        .context("alloc GCM mode")?;
    let block_modes_array = env
        .new_object_array(1, jni_str!("java/lang/String"), &block_modes)
        .context("alloc String[] for block modes")?;
    env.call_method(
        &builder,
        jni_str!("setBlockModes"),
        jni_sig!(
            (modes: ["java.lang.String"]) -> "android.security.keystore.KeyGenParameterSpec$Builder"
        ),
        &[JValue::Object(&block_modes_array)],
    )
    .context("Builder.setBlockModes")?;

    let padding = env
        .new_string("NoPadding")
        .context("alloc NoPadding")?;
    let padding_array = env
        .new_object_array(1, jni_str!("java/lang/String"), &padding)
        .context("alloc String[] for padding")?;
    env.call_method(
        &builder,
        jni_str!("setEncryptionPaddings"),
        jni_sig!(
            (paddings: ["java.lang.String"])
                -> "android.security.keystore.KeyGenParameterSpec$Builder"
        ),
        &[JValue::Object(&padding_array)],
    )
    .context("Builder.setEncryptionPaddings")?;

    env.call_method(
        &builder,
        jni_str!("setKeySize"),
        jni_sig!(
            (size: jint) -> "android.security.keystore.KeyGenParameterSpec$Builder"
        ),
        &[JValue::Int(256)],
    )
    .context("Builder.setKeySize(256)")?;

    let spec = env
        .call_method(
            &builder,
            jni_str!("build"),
            jni_sig!(() -> "android.security.keystore.KeyGenParameterSpec"),
            &[],
        )
        .context("Builder.build")?
        .l()
        .context("Builder.build returned non-object")?;

    // KeyGenerator gen = KeyGenerator.getInstance("AES", "AndroidKeyStore");
    let aes = env.new_string("AES").context("alloc AES")?;
    let provider = env
        .new_string("AndroidKeyStore")
        .context("alloc provider")?;
    let gen_class = env
        .find_class(jni_str!("javax/crypto/KeyGenerator"))
        .context("FindClass KeyGenerator")?;
    let generator = env
        .call_static_method(
            &gen_class,
            jni_str!("getInstance"),
            jni_sig!(
                (algorithm: "java.lang.String", provider: "java.lang.String")
                    -> "javax.crypto.KeyGenerator"
            ),
            &[JValue::Object(&aes), JValue::Object(&provider)],
        )
        .context("KeyGenerator.getInstance(AES, AndroidKeyStore)")?
        .l()
        .context("KeyGenerator.getInstance returned non-object")?;

    env.call_method(
        &generator,
        jni_str!("init"),
        jni_sig!((spec: "java.security.spec.AlgorithmParameterSpec") -> void),
        &[JValue::Object(&spec)],
    )
    .context("KeyGenerator.init(spec)")?;

    env.call_method(
        &generator,
        jni_str!("generateKey"),
        jni_sig!(() -> "javax.crypto.SecretKey"),
        &[],
    )
    .context("KeyGenerator.generateKey")?;
    Ok(())
}

fn delete_key<'local>(env: &mut Env<'local>, alias: &str) -> Result<()> {
    let keystore = open_keystore(env)?;
    let alias_jstr = env.new_string(alias).context("alloc alias")?;
    env.call_method(
        &keystore,
        jni_str!("deleteEntry"),
        jni_sig!((alias: "java.lang.String") -> void),
        &[JValue::Object(&alias_jstr)],
    )
    .context("KeyStore.deleteEntry")?;
    Ok(())
}

fn fetch_secret_key<'local>(env: &mut Env<'local>, alias: &str) -> Result<JObject<'local>> {
    let keystore = open_keystore(env)?;
    let alias_jstr = env.new_string(alias).context("alloc alias")?;
    let entry = env
        .call_method(
            &keystore,
            jni_str!("getKey"),
            jni_sig!(
                (alias: "java.lang.String", password: [jchar])
                    -> "java.security.Key"
            ),
            &[JValue::Object(&alias_jstr), JValue::Object(&JObject::null())],
        )
        .context("KeyStore.getKey")?
        .l()
        .context("getKey returned non-object")?;
    if entry.is_null() {
        return Err(anyhow!("AndroidKeyStore alias {alias} not found"));
    }
    Ok(entry)
}

fn encrypt<'local>(
    env: &mut Env<'local>,
    alias: &str,
    plaintext: &[u8],
) -> Result<(Vec<u8>, Vec<u8>)> {
    let key = fetch_secret_key(env, alias)?;
    let cipher = init_cipher(env, &key, /* encrypt */ true, None)?;
    // ciphertext = cipher.doFinal(plaintext)
    let pt_array = new_byte_array(env, plaintext)?;
    let ct = env
        .call_method(
            &cipher,
            jni_str!("doFinal"),
            jni_sig!((bytes: [jbyte]) -> [jbyte]),
            &[JValue::Object(&pt_array)],
        )
        .context("Cipher.doFinal(encrypt)")?
        .l()
        .context("doFinal returned non-object")?;
    let ct_array: jni::objects::JByteArray<'_> = env
        .cast_local::<jni::objects::JByteArray>(ct)
        .context("doFinal output not byte[]")?;
    let ciphertext = byte_array_to_vec(env, &ct_array)?;

    // iv = cipher.getIV()
    let iv = env
        .call_method(&cipher, jni_str!("getIV"), jni_sig!(() -> [jbyte]), &[])
        .context("Cipher.getIV")?
        .l()
        .context("getIV returned non-object")?;
    let iv_array: jni::objects::JByteArray<'_> = env
        .cast_local::<jni::objects::JByteArray>(iv)
        .context("getIV not byte[]")?;
    let iv_bytes = byte_array_to_vec(env, &iv_array)?;
    Ok((iv_bytes, ciphertext))
}

fn decrypt<'local>(
    env: &mut Env<'local>,
    alias: &str,
    iv: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>> {
    let key = fetch_secret_key(env, alias)?;
    let cipher = init_cipher(env, &key, /* encrypt */ false, Some(iv))?;
    let ct_array = new_byte_array(env, ciphertext)?;
    let pt = env
        .call_method(
            &cipher,
            jni_str!("doFinal"),
            jni_sig!((bytes: [jbyte]) -> [jbyte]),
            &[JValue::Object(&ct_array)],
        )
        .context("Cipher.doFinal(decrypt)")?
        .l()
        .context("doFinal returned non-object")?;
    let pt_array: jni::objects::JByteArray<'_> = env
        .cast_local::<jni::objects::JByteArray>(pt)
        .context("doFinal output not byte[]")?;
    byte_array_to_vec(env, &pt_array)
}

fn init_cipher<'local>(
    env: &mut Env<'local>,
    key: &JObject<'local>,
    encrypt: bool,
    iv: Option<&[u8]>,
) -> Result<JObject<'local>> {
    // Cipher cipher = Cipher.getInstance("AES/GCM/NoPadding");
    let transform = env
        .new_string("AES/GCM/NoPadding")
        .context("alloc transform")?;
    let cipher_class = env
        .find_class(jni_str!("javax/crypto/Cipher"))
        .context("FindClass Cipher")?;
    let cipher = env
        .call_static_method(
            &cipher_class,
            jni_str!("getInstance"),
            jni_sig!((transform: "java.lang.String") -> "javax.crypto.Cipher"),
            &[JValue::Object(&transform)],
        )
        .context("Cipher.getInstance")?
        .l()
        .context("Cipher.getInstance returned non-object")?;

    let mode = if encrypt { 1 } else { 2 }; // ENCRYPT_MODE / DECRYPT_MODE

    if let Some(iv_bytes) = iv {
        let iv_array = new_byte_array(env, iv_bytes)?;
        let spec_class = env
            .find_class(jni_str!("javax/crypto/spec/GCMParameterSpec"))
            .context("FindClass GCMParameterSpec")?;
        let spec = env
            .new_object(
                &spec_class,
                jni_sig!((tag_len_bits: jint, iv: [jbyte]) -> void),
                &[JValue::Int(128), JValue::Object(&iv_array)],
            )
            .context("new GCMParameterSpec")?;
        env.call_method(
            &cipher,
            jni_str!("init"),
            jni_sig!(
                (mode: jint, key: "java.security.Key", spec: "java.security.spec.AlgorithmParameterSpec")
                    -> void
            ),
            &[JValue::Int(mode), JValue::Object(key), JValue::Object(&spec)],
        )
        .context("Cipher.init(decrypt)")?;
    } else {
        env.call_method(
            &cipher,
            jni_str!("init"),
            jni_sig!((mode: jint, key: "java.security.Key") -> void),
            &[JValue::Int(mode), JValue::Object(key)],
        )
        .context("Cipher.init(encrypt)")?;
    }
    Ok(cipher)
}
