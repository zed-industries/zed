use sqlx::{Error, PgPool};

use std::{cmp, str};

// https://rustsec.org/advisories/RUSTSEC-2024-0363.html
#[sqlx::test(migrations = false, fixtures("./fixtures/rustsec/2024_0363.sql"))]
async fn rustsec_2024_0363(pool: PgPool) -> anyhow::Result<()> {
    let overflow_len = 4 * 1024 * 1024 * 1024; // 4 GiB

    // These three strings concatenated together will be the first query the Postgres backend "sees"
    //
    // Rather contrived because this already represents an injection vulnerability,
    // but it's easier to demonstrate the bug with a simple `Query` message
    // than the `Prepare` -> `Bind` -> `Execute` flow.
    let real_query_prefix = "INSERT INTO injection_target(message) VALUES ('";
    let fake_message = "fake_msg') RETURNING id;\0";
    let real_query_suffix = "') RETURNING id";

    // Our payload is another simple `Query` message
    let real_payload =
        "Q\0\0\0\x4DUPDATE injection_target SET message = 'you''ve been pwned!' WHERE id = 1\0";

    // This is the value we want the length prefix to overflow to (including the length of the prefix itself)
    // This will leave the backend's buffer pointing at our real payload.
    let fake_payload_len = real_query_prefix.len() + fake_message.len() + 4;

    // Pretty easy to see that this should overflow to `fake_payload_len`
    let target_payload_len = overflow_len + fake_payload_len;

    // This is the length we expect `injected_value` to be
    let expected_inject_len = target_payload_len
        - 4 // Length prefix
        - real_query_prefix.len()
        - (real_query_suffix.len() + 1 /* NUL terminator */);

    let pad_to_len = expected_inject_len - 5; // Header for FLUSH message that eats `real_query_suffix` (see below)

    let expected_payload_len = 4 // length prefix
        + real_query_prefix.len()
        + expected_inject_len
        + real_query_suffix.len()
        + 1; // NUL terminator

    let expected_wrapped_len = expected_payload_len % overflow_len;
    assert_eq!(expected_wrapped_len, fake_payload_len);

    // This will be the string we inject into the query.
    let mut injected_value = String::with_capacity(expected_inject_len);

    injected_value.push_str(fake_message);
    injected_value.push_str(real_payload);

    // The Postgres backend reads the `FLUSH` message but ignores its contents.
    // This gives us a variable-length NOP that lets us pad to the length we want,
    // as well as a way to eat `real_query_suffix` without breaking the connection.
    let flush_fill = "\0".repeat(9996);

    let flush_fmt_code = 'H'; // note: 'F' is `FunctionCall`.

    'outer: while injected_value.len() < pad_to_len {
        let remaining_len = pad_to_len - injected_value.len();

        // The max length of a FLUSH message is 10,000, including the length prefix.
        let flush_len = cmp::min(
            remaining_len - 1, // minus format code
            10000,
        );

        // We need `flush_len` to be valid UTF-8 when encoded in big-endian
        // in order to push it to the string.
        //
        // Not every value is going to be valid though, so we search for one that is.
        'inner: for flush_len in (4..=flush_len).rev() {
            let flush_len_be = (flush_len as i32).to_be_bytes();

            let Ok(flush_len_str) = str::from_utf8(&flush_len_be) else {
                continue 'inner;
            };

            let fill_len = flush_len - 4;

            injected_value.push(flush_fmt_code);
            injected_value.push_str(flush_len_str);
            injected_value.push_str(&flush_fill[..fill_len]);

            continue 'outer;
        }

        panic!("unable to find a valid encoding/split for {flush_len}");
    }

    assert_eq!(injected_value.len(), pad_to_len);

    // The amount of data the last FLUSH message has to eat
    let eat_len = real_query_suffix.len() + 1; // plus NUL terminator

    // Push the FLUSH message that will eat `real_query_suffix`
    injected_value.push(flush_fmt_code);
    injected_value.push_str(str::from_utf8(&(eat_len as i32).to_be_bytes()).unwrap());
    // The value will be in the buffer already.

    assert_eq!(expected_inject_len, injected_value.len());

    let query = format!("{real_query_prefix}{injected_value}{real_query_suffix}");

    // The length of the `Query` message we've created
    let final_payload_len = 4 // length prefix
        + query.len()
        + 1; // NUL terminator

    assert_eq!(expected_payload_len, final_payload_len);

    let wrapped_len = final_payload_len % overflow_len;

    assert_eq!(wrapped_len, fake_payload_len);

    let res = sqlx::raw_sql(&query)
        // Note: the connection may hang afterward
        // because `pending_ready_for_query_count` will underflow.
        .execute(&pool)
        .await;

    if let Err(e) = res {
        // Connection rejected the query; we're happy.
        if matches!(e, Error::Protocol(_)) {
            return Ok(());
        }

        panic!("unexpected error: {e:?}");
    }

    let messages: Vec<String> =
        sqlx::query_scalar("SELECT message FROM injection_target ORDER BY id")
            .fetch_all(&pool)
            .await?;

    // If the injection succeeds, `messages` will look like:
    // ["you've been pwned!'.to_string(), "fake_msg".to_string()]
    assert_eq!(
        messages,
        ["existing message".to_string(), "fake_msg".to_string()]
    );

    // Injection didn't affect our database; we're happy.
    Ok(())
}
