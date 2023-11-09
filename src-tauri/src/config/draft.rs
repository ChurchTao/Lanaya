use super::CommonConfig;
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use std::sync::Arc;

// 草稿，用于修改配置，T=正式数据 Option<T>=草稿数据
#[derive(Debug, Clone)]
pub struct Draft<T: Clone + ToOwned> {
    inner: Arc<Mutex<(T, Option<T>)>>,
}

macro_rules! draft_define {
    ($id: ident) => {
        impl Draft<$id> {
            // 获取正式数据
            #[allow(unused)]
            pub fn data(&self) -> MappedMutexGuard<$id> {
                MutexGuard::map(self.inner.lock(), |guard| &mut guard.0)
            }

            // 获取最后修改记录
            pub fn latest(&self) -> MappedMutexGuard<$id> {
                MutexGuard::map(self.inner.lock(), |inner| {
                    if inner.1.is_none() {
                        &mut inner.0
                    } else {
                        inner.1.as_mut().unwrap()
                    }
                })
            }

            // 获取草稿数据
            pub fn draft(&self) -> MappedMutexGuard<$id> {
                MutexGuard::map(self.inner.lock(), |inner| {
                    if inner.1.is_none() {
                        inner.1 = Some(inner.0.clone());
                    }
                    inner.1.as_mut().unwrap()
                })
            }

            // 同步草稿数据到正式数据
            pub fn apply(&self) -> Option<$id> {
                let mut inner = self.inner.lock();
                match inner.1.take() {
                    Some(draft) => {
                        let old_value = inner.0.to_owned();
                        inner.0 = draft.to_owned();
                        Some(old_value)
                    }
                    None => None,
                }
            }

            // 丢弃草稿数据
            #[allow(unused)]
            pub fn discard(&self) -> Option<$id> {
                let mut inner = self.inner.lock();
                inner.1.take()
            }
        }

        impl From<$id> for Draft<$id> {
            fn from(data: $id) -> Self {
                Draft {
                    inner: Arc::new(Mutex::new((data, None))),
                }
            }
        }
    };
}

draft_define!(CommonConfig);
