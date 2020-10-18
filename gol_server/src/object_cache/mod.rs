// created from https://github.com/hyperledger/indy-sdk/tree/master/vcx/libvcx

use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Mutex;
use std::sync::MutexGuard;

use rand::Rng;

use crate::error::prelude::*;

pub struct ObjectCache<T> {
    pub cache_name: String,
    pub store: Mutex<HashMap<u32, Mutex<T>>>,
}

impl<T> ObjectCache<T> {
    pub fn new(cache_name: &str) -> ObjectCache<T> {
        ObjectCache {
            store: Default::default(),
            cache_name: cache_name.to_string(),
        }
    }

    fn _lock_store(&self) -> GolResult<MutexGuard<HashMap<u32, Mutex<T>>>> {
        match self.store.lock() {
            Ok(g) => Ok(g),
            Err(e) => {
                error!("Unable to lock Object Store: {:?}", e);
                Err(GolServerError::from_msg(GolServerErrorKind::Common(10), format!("[ObjectCache: {}] Unable to lock Object Store: {:?}", self.cache_name, e)))
            }
        }
    }

    pub fn has_handle(&self, handle: u32) -> bool {
        let store = match self._lock_store() {
            Ok(g) => g,
            Err(_) => return false
        };
        store.contains_key(&handle)
    }

    pub fn get<F, R>(&self, handle: u32, closure: F) -> GolResult<R>
        where F: Fn(&T) -> GolResult<R> {
        let store = self._lock_store()?;
        match store.get(&handle) {
            Some(m) => match m.lock() {
                Ok(obj) => closure(obj.deref()),
                Err(_) => Err(GolServerError::from_msg(GolServerErrorKind::Common(10), format!("[ObjectCache: {}] Unable to lock Object Store", self.cache_name))) //TODO better error
            },
            None => Err(GolServerError::from_msg(GolServerErrorKind::ServerError, format!("[ObjectCache: {}] Object not found for handle: {}", self.cache_name, handle)))
        }
    }

    pub fn get_mut<F, R>(&self, handle: u32, closure: F) -> GolResult<R>
        where F: Fn(&mut T) -> GolResult<R> {
        let mut store = self._lock_store()?;
        match store.get_mut(&handle) {
            Some(m) => match m.lock() {
                Ok(mut obj) => closure(obj.deref_mut()),
                Err(_) => Err(GolServerError::from_msg(GolServerErrorKind::Common(10), format!("[ObjectCache: {}] Unable to lock Object Store", self.cache_name))) //TODO better error
            },
            None => Err(GolServerError::from_msg(GolServerErrorKind::ServerError, format!("[ObjectCache: {}] Object not found for handle: {}", self.cache_name, handle)))
        }
    }

    pub fn add(&self, obj: T) -> GolResult<u32> {
        let mut store = self._lock_store()?;

        let mut new_handle = rand::thread_rng().gen::<u32>();
        loop {
            if !store.contains_key(&new_handle) {
                break;
            }
            new_handle = rand::thread_rng().gen::<u32>();
        }

        match store.insert(new_handle, Mutex::new(obj)) {
            Some(_) => Ok(new_handle),
            None => Ok(new_handle)
        }
    }

    pub fn insert(&self, handle: u32, obj: T) -> GolResult<()> {
        let mut store = self._lock_store()?;

        match store.insert(handle, Mutex::new(obj)) {
            _ => Ok(()),
        }
    }

    pub fn release(&self, handle: u32) -> GolResult<()> {
        let mut store = self._lock_store()?;
        match store.remove(&handle) {
            Some(_) => Ok(()),
            None => Err(GolServerError::from_msg(GolServerErrorKind::ServerError, format!("[ObjectCache: {}] Object not found for handle: {}", self.cache_name, handle)))
        }
    }

    pub fn drain(&self) -> GolResult<()> {
        let mut store = self._lock_store()?;
        Ok(store.clear())
    }
}

#[cfg(test)]
mod tests {
    use object_cache::ObjectCache;
    use utils::devsetup::SetupDefaults;
    use crate::server::object_cache::ObjectCache;
    use crate::object_cache::ObjectCache;

    #[test]
    #[cfg(feature = "general_test")]
    fn create_test() {
        let _setup = SetupDefaults::init();

        let _c: ObjectCache<u32> = ObjectCache::new("cache0-u32");
    }

    #[test]
    #[cfg(feature = "general_test")]
    fn get_closure() {
        let _setup = SetupDefaults::init();

        let test: ObjectCache<u32> = ObjectCache::new("cache1-u32");
        let handle = test.add(2222).unwrap();
        let rtn = test.get(handle, |obj| Ok(obj.clone()));
        assert_eq!(2222, rtn.unwrap())
    }

    #[test]
    #[cfg(feature = "general_test")]
    fn to_string_test() {
        let _setup = SetupDefaults::init();

        let test: ObjectCache<u32> = ObjectCache::new("cache2-u32");
        let handle = test.add(2222).unwrap();
        let string: String = test.get(handle, |_| {
            Ok(String::from("TEST"))
        }).unwrap();

        assert_eq!("TEST", string);
    }

    #[test]
    #[cfg(feature = "general_test")]
    fn mut_object_test() {
        let _setup = SetupDefaults::init();

        let test: ObjectCache<String> = ObjectCache::new("cache3-string");
        let handle = test.add(String::from("TEST")).unwrap();

        test.get_mut(handle, |obj| {
            obj.to_lowercase();
            Ok(())
        }).unwrap();

        let string: String = test.get(handle, |obj| {
            Ok(obj.clone())
        }).unwrap();

        assert_eq!("TEST", string);
    }
}
