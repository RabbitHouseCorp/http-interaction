use std::fmt;

pub struct InteractionData {
    pub application_id: String,
    pub _type: u64,
    pub id: String,
    pub token: String,
}

impl std::fmt::Debug for InteractionData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InteractionData")
            .field("application_id", &self.application_id)
            .field("_type", &self._type)
            .field("id", &self.id)
            .field("token", &self.token)
            .finish()
    }
}
