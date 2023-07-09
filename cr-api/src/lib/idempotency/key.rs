// src/lib/idempotency/key.rs

// struct to wrap a string representing the idempotency key
#[derive(Debug)]
pub struct IdempotencyKey(String);

// implement the TryFrom trait to convert a string into our Idempotency Key type
impl TryFrom<String> for IdempotencyKey {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.is_empty() {
            anyhow::bail!("The idempotency key cannot by empty");
        }
        let max_length = 50;
        if s.len() >= max_length {
            anyhow::bail!("The idempotency key must be shorter than {max_length} characters");
        }
        Ok(Self(s))
    }
}

// implement the From trait to convert an Idempotency Key type to a string
impl From<IdempotencyKey> for String {
    fn from(k: IdempotencyKey) -> Self {
        k.0
    }
}

// implement AsRef for our Idempotency Key type to get the inner value
impl AsRef<str> for IdempotencyKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
