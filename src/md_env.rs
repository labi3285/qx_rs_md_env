use qx_rs_err::err::*;
use qx_rs_env::env;

use std::{str::FromStr, sync::{Arc, RwLock}};
use lazy_static::lazy_static;

use crate::env_entry::EnvEntry;


pub fn setup(debug_env: Option<&str>) -> Result<()> {
    let mut env_name = ".env".to_string();
    if cfg!(debug_assertions) {
        if let Some(v) = debug_env {
            env_name = format!(".env.{}", v);
        }
    }
    let env = _load_env(&env_name)?;
    _set_cache(env)?;
    Ok(())
}

pub fn str(key: &str) -> Option<String> {
    _env().str(key)
}
pub fn strs(key: &str, separator: &str) -> Option<Vec<String>> {
    _env().strs(key, separator)
}
pub fn val<F: FromStr>(key: &str) -> Result<Option<F>> {
    _env().val(key)
}
pub fn vals<F: FromStr>( key: &str, separator: &str) -> Result<Option<Vec<F>>> {
    _env().vals(key, separator)
}


lazy_static! {
    static ref __CACHE__: RwLock<Option<EnvEntry>> = RwLock::new(None);
}
fn _get_cache() -> Result<Option<EnvEntry>> {
    let guard = __CACHE__.read().map_err(|err| {
        Error::error(Box::new(err))
    })?;
    if let Some(a) = &*guard {
        let e = a.clone();
        Ok(Some(e.clone()))
    } else {
        Ok(None)
    }
}
fn _set_cache(env: EnvEntry) -> Result<()> {
    let mut guard = __CACHE__.write().map_err(|err| {
        Error::error(Box::new(err))
    })?;
    *guard = Some(env);
    Ok(())
}
fn _load_env(env_name: &str) -> Result<EnvEntry> {
    let _env = env::Env::new(env_name)?;
    if let Some(e) = _env {
        Ok(EnvEntry { _env: Some(Arc::new(e)) })
    } else {
        Ok(EnvEntry { _env: None })
    }
}
fn _env() -> EnvEntry {
    let env = _get_cache().expect("setup needed").expect("setup needed");
    env
}