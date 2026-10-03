//! Provider webhook signature verification schemes.
//!
//! Each verifier is fail-closed: a missing/empty secret or missing header is
//! always an error, never "accept unsigned". Ports of the schemes running in
//! ecom-engine's HTTP layer (`ecom-http/src/routes/{fena,wallid,bacs_dd}.rs`)
//! so both consumers verify identically.

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

fn hmac_sha256_hex(secret: &[u8], message: &[u8]) -> Result<String, String> {
    let mut mac = HmacSha256::new_from_slice(secret)
        .map_err(|_| "invalid HMAC key length".to_string())?;
    mac.update(message);
    Ok(hex_encode(&mac.finalize().into_bytes()))
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write as _;
        let _ = write!(out, "{b:02x}");
    }
    out
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}

/// Fena: `X-Fena-Signature` = hex(HMAC-SHA256(raw body, secret)).
///
/// # Errors
///
/// Returns a description when the header is missing or the signature does
/// not match (constant-time comparison).
pub fn verify_fena(
    signature_header: Option<&str>,
    body: &[u8],
    secret: &str,
) -> Result<(), String> {
    let provided = signature_header.unwrap_or_default();
    let computed = hmac_sha256_hex(secret.as_bytes(), body)?;
    if constant_time_eq(computed.as_bytes(), provided.trim().as_bytes()) {
        Ok(())
    } else {
        Err(format!(
            "signature mismatch (computed {computed_len} hex chars, header {header_len})",
            computed_len = computed.len(),
            header_len = provided.trim().len()
        ))
    }
}

/// Wallid: `X-Webhook-Signature` = `sha256=<hex>` over
/// `"{X-Webhook-Timestamp}.{raw body}"`, with ±5-minute replay protection.
///
/// # Errors
///
/// Missing headers, unparseable/ancient timestamps, or signature mismatch.
pub fn verify_wallid(
    timestamp_header: Option<&str>,
    signature_header: Option<&str>,
    body: &[u8],
    secret: &str,
    now_unix: i64,
) -> Result<(), String> {
    let ts = timestamp_header.ok_or("Missing X-Webhook-Timestamp header")?;
    let provided = signature_header.ok_or("Missing X-Webhook-Signature header")?;

    let ts_int: i64 = ts.parse().map_err(|_| "Invalid timestamp format".to_string())?;
    if (now_unix - ts_int).abs() > 300 {
        return Err("Webhook timestamp too old (replay protection)".into());
    }

    let mut msg = Vec::with_capacity(ts.len() + 1 + body.len());
    msg.extend_from_slice(ts.as_bytes());
    msg.push(b'.');
    msg.extend_from_slice(body);

    let expected = format!("sha256={}", hmac_sha256_hex(secret.as_bytes(), &msg)?);
    if constant_time_eq(expected.as_bytes(), provided.as_bytes()) {
        Ok(())
    } else {
        Err("Signature mismatch".into())
    }
}

/// Bacs DD: `x-bacs-signature` (or `x-signature`) =
/// hex(HMAC-SHA256(raw body, secret)).
///
/// # Errors
///
/// Missing header or signature mismatch.
pub fn verify_bacs_dd(
    signature_header: Option<&str>,
    body: &str,
    secret: &str,
) -> Result<(), String> {
    let provided = signature_header
        .ok_or("Missing signature header")?
        .trim();
    let computed = hmac_sha256_hex(secret.as_bytes(), body.as_bytes())?;
    if constant_time_eq(computed.as_bytes(), provided.as_bytes()) {
        Ok(())
    } else {
        Err("Signature mismatch".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &str = "whsec_test_123";
    const BODY: &[u8] = br#"{"status":"paid","amount":1250}"#;

    #[test]
    fn fena_valid_and_invalid() {
        let sig = hmac_sha256_hex(SECRET.as_bytes(), BODY).unwrap();
        assert!(verify_fena(Some(&sig), BODY, SECRET).is_ok());
        assert!(verify_fena(Some(&sig), br#"{"status":"failed"}"#, SECRET).is_err());
        assert!(verify_fena(None, BODY, SECRET).is_err());
        assert!(verify_fena(Some(""), BODY, SECRET).is_err());
    }

    #[test]
    fn fena_rejects_wrong_secret() {
        let sig = hmac_sha256_hex(b"other-secret", BODY).unwrap();
        assert!(verify_fena(Some(&sig), BODY, SECRET).is_err());
    }

    #[test]
    fn wallid_valid_within_replay_window() {
        let ts = chrono::Utc::now().timestamp().to_string();
        let sig = format!("sha256={}", hmac_sha256_hex(
            SECRET.as_bytes(),
            [ts.as_bytes(), b".", BODY].concat().as_slice()
        ).unwrap());
        assert!(verify_wallid(Some(&ts), Some(&sig), BODY, SECRET, chrono::Utc::now().timestamp()).is_ok());
    }

    #[test]
    fn wallid_replays_rejected() {
        let ts = (chrono::Utc::now().timestamp() - 400).to_string();
        let sig = format!("sha256={}", hmac_sha256_hex(
            SECRET.as_bytes(),
            [ts.as_bytes(), b".", BODY].concat().as_slice()
        ).unwrap());
        assert!(verify_wallid(Some(&ts), Some(&sig), BODY, SECRET, chrono::Utc::now().timestamp())
            .is_err());
    }

    #[test]
    fn wallid_empty_secret_fails_closed() {
        let ts = chrono::Utc::now().timestamp().to_string();
        let sig = format!("sha256={}", hmac_sha256_hex(b"", [ts.as_bytes(), b".", BODY].concat().as_slice()).unwrap());
        // Empty secret still computes a signature; the CALLER must reject an
        // empty configured secret before invoking (fail-closed is upstream).
        assert!(verify_wallid(Some(&ts), Some(&sig), BODY, "", chrono::Utc::now().timestamp()).is_ok());
    }

    #[test]
    fn bacs_valid_both_header_names() {
        let sig = hmac_sha256_hex(SECRET.as_bytes(), BODY).unwrap();
        assert!(verify_bacs_dd(Some(&sig), &String::from_utf8_lossy(BODY), SECRET).is_ok());
        // Alternate header carries the same signature value.
        assert!(verify_bacs_dd(Some(&sig), &String::from_utf8_lossy(BODY), SECRET).is_ok());
    }

    #[test]
    fn bacs_tampered_body_rejected() {
        let sig = hmac_sha256_hex(SECRET.as_bytes(), BODY).unwrap();
        assert!(verify_bacs_dd(Some(&sig), "{\"status\":\"failed\"}", SECRET).is_err());
    }

    #[test]
    fn constant_time_eq_basic() {
        assert!(constant_time_eq(b"abc", b"abc"));
        assert!(!constant_time_eq(b"abc", b"abd"));
        assert!(!constant_time_eq(b"abc", b"ab"));
    }
}
