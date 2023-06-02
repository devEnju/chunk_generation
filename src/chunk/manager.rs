use super::provider::Provider;

pub struct Manager<T: Provider> {
    pub provider: T,
}

impl<T: Provider> Manager<T> {
    pub fn new(provider: T) -> Self {
        Manager { provider }
    }
}
