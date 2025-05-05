use std::{str::FromStr, sync::Arc};

use qx_rs_err::err::*;
use qx_rs_env::env;


#[derive(Debug, Clone)]
pub(crate) struct EnvEntry {
    pub(crate) _env: Option<Arc<env::Env>>,
}
impl EnvEntry {
    pub(crate) fn str(&self, key: &str) -> Option<String> {
        if let Some(a) = self._env.clone() {
            a.str(key)
        } else {
            None
        }
    }
    pub(crate) fn strs(&self, key: &str, separator: &str) -> Option<Vec<String>> {
        if let Some(a) = self._env.clone() {
            a.strs(key, separator)
        } else {
            None
        }
    }
}
impl EnvEntry {
    pub(crate) fn val<F: FromStr>(&self, key: &str) -> Result<Option<F>> {
        if let Some(a) = self._env.clone() {
            a.val(key)
        } else {
            Ok(None)
        }
    }
    pub(crate) fn vals<F: FromStr>(&self, key: &str, separator: &str) -> Result<Option<Vec<F>>> {
        if let Some(a) = self._env.clone() {
            a.vals(key, separator)
        } else {
            Ok(None)
        }
    }
}