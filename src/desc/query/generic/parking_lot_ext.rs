use crate::desc::{ParsedField, Query, QueryMut};
use parking_lot::{FairMutex, Mutex, RwLock};

macro_rules! impl_lock {
    ($wrapper:ident, $lock:ident) => {
        impl<'a, Querier: QueryMut<'a>> Query<'a> for $wrapper<Querier> {
            fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
                self.$lock().query_raw_text_mut(field)
            }
        }

        impl<'a, Querier: QueryMut<'a>> QueryMut<'a> for $wrapper<Querier> {
            fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
                self.query_raw_text(field)
            }
        }
    };
}

impl_lock!(Mutex, lock);
impl_lock!(FairMutex, lock);
impl_lock!(RwLock, write);
