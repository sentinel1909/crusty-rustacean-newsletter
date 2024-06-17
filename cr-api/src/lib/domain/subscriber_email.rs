// src/lib/domain/subscriber_email.rs

// domain subscriber email type

// dependencies
use validator::{Validate, ValidateEmail};

// a struct to represent a subscriber email type
#[derive(Debug, Clone, Validate)]
pub struct SubscriberEmail {
    #[validate(email)]
    mail: String,
}

// impl block for the subscriber email type; contains a method to validate subscriber emails
impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if s.validate_email() {
            Ok(Self { mail: s })
        } else {
            Err(format!("{} is not a valid subscriber email.", s))
        }
    }
}

// impl block to return the inner value of the subscriber email type
impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.mail
    }
}

// impl block to implement the Display trait for the subscriber email type
impl std::fmt::Display for SubscriberEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We just forward to the Display implementation of
        // the wrapped String.
        self.mail.fmt(f)
    }
}

// unit tests for the subscriber email type
#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claims::assert_err;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }
}
