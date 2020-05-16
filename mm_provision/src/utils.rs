use std::collections::HashSet;
use std::ops::Sub;

pub struct BrokerAccess {
    given: HashSet<String>,
    revoked: HashSet<String>,
}

impl BrokerAccess {
    pub fn new() -> Self {
        BrokerAccess {
            given: HashSet::new(),
            revoked: HashSet::new(),
        }
    }

    pub fn extend(&mut self, other: &BrokerAccess) {
        self.given.extend(other.needs_login_access());
        self.revoked.extend(other.needs_revoked_access());
    }

    pub fn mark_access(&mut self, tokens: &Option<Vec<String>>) {
        if let Some(tokens) = tokens {
            self.given.extend(tokens.clone());
        }
    }

    pub fn mark_revoked(&mut self, tokens: &Option<Vec<String>>) {
        if let Some(tokens) = tokens {
            self.revoked.extend(tokens.clone());
        }
    }

    pub fn needs_login_access(&self) -> HashSet<String> {
        self.given.clone()
    }

    pub fn needs_revoked_access(&self) -> HashSet<String> {
        self.revoked.sub(&self.given)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal() {
        let mut broker_access = BrokerAccess::new();

        fn token(s: &str) -> String {
            String::from(s)
        }

        // Mark that some tokens has access
        broker_access.mark_access(&Some(vec![token("a"), token("b"), token("c")]));
        broker_access.mark_access(&Some(vec![token("d")]));

        // Mark that some tokens had their access revoked
        broker_access.mark_revoked(&Some(vec![token("a"), token("p"), token("q")]));

        // Extract and test those tokens that need login access
        let needs_login_access = broker_access.needs_login_access();
        assert_eq!(needs_login_access.len(), 4);
        assert!(needs_login_access.contains(&token("a")));
        assert!(needs_login_access.contains(&token("b")));
        assert!(needs_login_access.contains(&token("c")));
        assert!(needs_login_access.contains(&token("d")));

        // Extract and test the tokens that need there access revoked
        let needs_revoked_access = broker_access.needs_revoked_access();
        assert_eq!(needs_revoked_access.len(), 2);
        assert!(needs_revoked_access.contains(&token("p")));
        assert!(needs_revoked_access.contains(&token("q")));

        // Especially a needs access, even if revoked once
        assert_eq!(needs_revoked_access.contains(&token("a")), false);
    }
}
