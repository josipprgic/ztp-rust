use crate::routes::SubReq;

pub struct SubscriberName(String);
#[derive(Clone)]
pub struct SubscriberEmail(String);

pub struct SubscriberDetails {
    pub name: SubscriberName,
    pub email: SubscriberEmail,
}

impl TryFrom<SubReq> for SubscriberDetails {
    type Error = String;

    fn try_from(req: SubReq) -> Result<SubscriberDetails, String> {
        let name = req.name.try_into()?;
        let email = req.email.try_into()?;

        Ok(SubscriberDetails { name, email })
    }
}

impl TryFrom<String> for SubscriberName {
    type Error = String;

    fn try_from(s: String) -> Result<SubscriberName, String> {
        if s.len() > 256 {
            return Err("Name too long".to_string());
        }

        if s.trim().is_empty() {
            return Err("Name is empty".to_string());
        }

        Ok(SubscriberName(s))
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for SubscriberEmail {
    type Error = String;

    fn try_from(s: String) -> Result<SubscriberEmail, String> {
        if s.len() > 256 {
            return Err("Email too long".to_string());
        }

        if s.find("@").is_none() || s.rfind("@").unwrap() != s.find("@").unwrap() {
            return Err("More than one @ symbol".to_string());
        }

        Ok(SubscriberEmail(s))
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_name() {}
}
