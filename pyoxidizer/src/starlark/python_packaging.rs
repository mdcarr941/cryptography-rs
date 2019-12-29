// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use starlark::environment::Environment;
use starlark::values::{default_compare, TypedValue, Value, ValueError, ValueResult};
use starlark::{
    any, immutable, not_supported, starlark_fun, starlark_module, starlark_signature,
    starlark_signature_extraction, starlark_signatures,
};
use std::any::Any;
use std::cmp::Ordering;
use std::collections::HashMap;

use super::env::required_str_arg;
use crate::app_packaging::config::PackagingWriteLicenseFiles;

#[derive(Debug, Clone)]
pub struct WriteLicenseFiles {
    pub rule: PackagingWriteLicenseFiles,
}

impl TypedValue for WriteLicenseFiles {
    immutable!();
    any!();
    not_supported!(binop);
    not_supported!(container);
    not_supported!(function);
    not_supported!(get_hash);
    not_supported!(to_int);

    fn to_str(&self) -> String {
        format!("WriteLicenseFiles<{:#?}>", self.rule)
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "WriteLicenseFiles"
    }

    fn to_bool(&self) -> bool {
        true
    }

    fn compare(&self, other: &dyn TypedValue, _recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }
}

starlark_module! { python_packaging_env =>
    #[allow(non_snake_case, clippy::ptr_arg)]
    WriteLicenseFiles(path) {
        let path = required_str_arg("path", &path)?;

        let rule = PackagingWriteLicenseFiles {
            path,
        };

        Ok(Value::new(WriteLicenseFiles { rule }))
    }
}

#[cfg(test)]
mod tests {
    use super::super::testutil::*;
    use super::*;

    #[test]
    fn test_write_license_files_default() {
        let err = starlark_nok("WriteLicenseFiles()");
        assert!(err.message.starts_with("Missing parameter path"));
    }

    #[test]
    fn test_write_license_files_basic() {
        let v = starlark_ok("WriteLicenseFiles('path')");
        let wanted = PackagingWriteLicenseFiles {
            path: "path".to_string(),
        };
        v.downcast_apply(|x: &WriteLicenseFiles| assert_eq!(x.rule, wanted));
    }
}
