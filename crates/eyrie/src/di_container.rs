use std::any::type_name;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static DI_CONTAINER: Lazy<Mutex<DiContainer>> = Lazy::new(|| Mutex::new(DiContainer::new()));

#[derive(Debug)]
pub enum Error {
    AlreadyRegistered { type_name: &'static str },
    NotRegistered { type_name: &'static str },
    TypeMismatch { expected: &'static str },
}

struct DiContainer {
    injectables: HashMap<TypeId, Arc<dyn Any + Send + Sync + 'static>>,
}

impl DiContainer {
    fn new() -> Self {
        Self {
            injectables: HashMap::new(),
        }
    }

    pub(crate) fn register<T>(&mut self, injectable: T) -> Result<(), Error>
    where
        T: Any + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();
        if self.injectables.contains_key(&type_id) {
            return Err(Error::AlreadyRegistered {
                type_name: type_name::<T>(),
            });
        }
        self.injectables.insert(type_id, Arc::new(injectable));
        Ok(())
    }

    pub(crate) fn resolve<T>(&self) -> Result<Arc<T>, Error>
    where
        T: Any + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();
        let Some(injectable) = self.injectables.get(&type_id) else {
            return Err(Error::NotRegistered {
                type_name: type_name::<T>(),
            });
        };
        let Ok(casted) = injectable.clone().downcast::<T>() else {
            return Err(Error::TypeMismatch {
                expected: type_name::<T>(),
            });
        };
        Ok(casted)
    }
}
