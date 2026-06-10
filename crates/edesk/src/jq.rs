//! Embedded jq filtering for `--jq`, powered by jaq (a Rust jq clone).

use anyhow::{anyhow, Result};
use jaq_core::load::{Arena, File, Loader};
use jaq_core::{data, unwrap_valr, Ctx, Vars};
use jaq_json::{Num, Val};
use serde_json::Value;

/// Run a jq `filter` over a single JSON `input`, returning all output values.
pub fn apply(filter: &str, input: Value) -> Result<Vec<Value>> {
    use serde::Deserialize as _;

    let input: Val =
        Val::deserialize(input).map_err(|e| anyhow!("failed to convert input: {e}"))?;

    let program = File {
        code: filter,
        path: (),
    };
    let defs = jaq_core::defs()
        .chain(jaq_std::defs())
        .chain(jaq_json::defs());
    let loader = Loader::new(defs);
    let arena = Arena::default();
    let modules = loader
        .load(&arena, program)
        .map_err(|errs| anyhow!("invalid jq expression: {errs:?}"))?;

    let funs = jaq_core::funs()
        .chain(jaq_std::funs())
        .chain(jaq_json::funs());
    let filter = jaq_core::Compiler::default()
        .with_funs(funs)
        .compile(modules)
        .map_err(|errs| anyhow!("invalid jq expression: {errs:?}"))?;

    let ctx = Ctx::<data::JustLut<Val>>::new(&filter.lut, Vars::new([]));
    filter
        .id
        .run((ctx, input))
        .map(unwrap_valr)
        .map(|r| r.map(val_to_json).map_err(|e| anyhow!("jq: {e}")))
        .collect()
}

/// jaq_json::Val -> serde_json::Value (jaq-json 2.0 provides no impl).
fn val_to_json(v: Val) -> Value {
    match v {
        Val::Null => Value::Null,
        Val::Bool(b) => Value::Bool(b),
        Val::Num(n) => num_to_json(n),
        Val::TStr(s) | Val::BStr(s) => Value::String(String::from_utf8_lossy(&s).into_owned()),
        Val::Arr(a) => Value::Array(a.iter().cloned().map(val_to_json).collect()),
        Val::Obj(o) => Value::Object(
            o.iter()
                .map(|(k, v)| (key_to_string(k), val_to_json(v.clone())))
                .collect(),
        ),
    }
}

/// Object keys in jaq-json 2.0 may be non-strings; JSON requires strings.
fn key_to_string(k: &Val) -> String {
    match k {
        Val::TStr(s) | Val::BStr(s) => String::from_utf8_lossy(s).into_owned(),
        other => other.to_string(),
    }
}

fn num_to_json(n: Num) -> Value {
    use serde_json::Number;
    let from_str = |s: &str| {
        s.parse::<i64>()
            .ok()
            .map(Number::from)
            .or_else(|| s.parse::<u64>().ok().map(Number::from))
            .or_else(|| s.parse::<f64>().ok().and_then(Number::from_f64))
            .map(Value::Number)
            .unwrap_or(Value::Null)
    };
    match n {
        Num::Int(i) => Value::Number(Number::from(i as i64)),
        Num::Float(f) => Number::from_f64(f)
            .map(Value::Number)
            .unwrap_or(Value::Null),
        Num::BigInt(b) => from_str(&b.to_string()),
        Num::Dec(d) => from_str(&d),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn identity() {
        let out = apply(".", json!({"a": 1})).unwrap();
        assert_eq!(out, vec![json!({"a": 1})]);
    }

    #[test]
    fn select_and_project() {
        let input = json!([{"id": 1, "ok": true}, {"id": 2, "ok": false}]);
        let out = apply(".[] | select(.ok) | .id", input).unwrap();
        assert_eq!(out, vec![json!(1)]);
    }

    #[test]
    fn invalid_expression_errors() {
        assert!(apply("|||", json!(null)).is_err());
    }
}
