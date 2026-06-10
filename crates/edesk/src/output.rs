//! Output rendering.
//!
//! Convention (borrowed from gh/rover): stdout carries data, stderr carries
//! everything else. Tables render when stdout is a TTY; piped output switches
//! to tab-separated values so `cut`/`awk` work. `--json` always emits raw API
//! JSON, `--jq` filters it, `--fields` projects columns/keys.

use std::io::{IsTerminal, Write};

use anyhow::Result;
use comfy_table::{presets, Cell, ContentArrangement, Table};
use edesk_client::Paginator;
use serde_json::Value;

use crate::cli::GlobalArgs;
use crate::jq;

/// A table column: header plus a dot-separated path into the JSON object.
#[derive(Debug, Clone, Copy)]
pub struct Column {
    pub header: &'static str,
    pub path: &'static str,
}

pub const fn col(header: &'static str, path: &'static str) -> Column {
    Column { header, path }
}

/// Print a list response.
pub fn print_list(
    global: &GlobalArgs,
    items: Vec<Value>,
    paginator: Option<Paginator>,
    columns: &[Column],
) -> Result<()> {
    let value = Value::Array(items);
    if let Some(expr) = &global.jq {
        return print_jq(expr, value);
    }
    if global.json {
        return print_json(&project(value, global.fields.as_deref()));
    }

    let Value::Array(items) = value else {
        unreachable!()
    };
    let selected = select_columns(columns, global.fields.as_deref());

    if std::io::stdout().is_terminal() {
        let mut table = Table::new();
        table
            .load_preset(presets::NOTHING)
            .set_content_arrangement(ContentArrangement::Dynamic);
        table.set_header(selected.iter().map(|c| Cell::new(c.header.to_uppercase())));
        for item in &items {
            table.add_row(selected.iter().map(|c| cell_text(item, &c.path)));
        }
        println!("{table}");
        if let Some(p) = paginator {
            let shown = items.len() as u64;
            if !global.quiet && p.total_items_count > shown {
                eprintln!(
                    "Showing {shown} of {} items (use --all, --limit or --page to fetch more)",
                    p.total_items_count
                );
            }
        }
    } else {
        let mut out = std::io::stdout().lock();
        for item in &items {
            let row: Vec<String> = selected.iter().map(|c| cell_text(item, &c.path)).collect();
            writeln!(out, "{}", row.join("\t"))?;
        }
    }
    Ok(())
}

/// Print a single-object response as a key/value listing (or JSON).
pub fn print_single(global: &GlobalArgs, value: Value) -> Result<()> {
    if let Some(expr) = &global.jq {
        return print_jq(expr, value);
    }
    if global.json {
        return print_json(&project(value, global.fields.as_deref()));
    }

    match &value {
        Value::Object(map) => {
            let mut table = Table::new();
            table
                .load_preset(presets::NOTHING)
                .set_content_arrangement(ContentArrangement::Dynamic);
            match global.fields.as_deref() {
                // Selected fields render in the order given, with dot-path
                // support (`user.name`).
                Some(fields) => {
                    for field in fields {
                        let val = lookup(&value, field).cloned().unwrap_or(Value::Null);
                        table.add_row(vec![Cell::new(field), Cell::new(display_value(&val))]);
                    }
                }
                None => {
                    for (key, val) in map {
                        table.add_row(vec![Cell::new(key), Cell::new(display_value(val))]);
                    }
                }
            }
            println!("{table}");
        }
        other => print_json(other)?,
    }
    Ok(())
}

/// Print a `{ok, message}` confirmation (deletes) to stderr, JSON to stdout
/// when requested.
pub fn print_confirmation(global: &GlobalArgs, value: Value) -> Result<()> {
    if let Some(expr) = &global.jq {
        return print_jq(expr, value);
    }
    if global.json {
        return print_json(&value);
    }
    let message = value
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or("done");
    if !global.quiet {
        eprintln!("✓ {message}");
    }
    Ok(())
}

pub fn print_json(value: &Value) -> Result<()> {
    let mut out = std::io::stdout().lock();
    serde_json::to_writer_pretty(&mut out, value)?;
    writeln!(out)?;
    Ok(())
}

fn print_jq(expr: &str, value: Value) -> Result<()> {
    let outputs = jq::apply(expr, value)?;
    let mut out = std::io::stdout().lock();
    for output in outputs {
        // Bare strings print raw (like `jq -r` for top-level strings in gh).
        match output {
            Value::String(s) => writeln!(out, "{s}")?,
            other => {
                serde_json::to_writer_pretty(&mut out, &other)?;
                writeln!(out)?;
            }
        }
    }
    Ok(())
}

/// Restrict table columns to `--fields` if given. Unknown fields become
/// ad-hoc columns so users can reach any JSON key, not just curated ones.
fn select_columns(columns: &[Column], fields: Option<&[String]>) -> Vec<OwnedColumn> {
    match fields {
        None => columns
            .iter()
            .map(|c| OwnedColumn {
                header: c.header.to_string(),
                path: c.path.to_string(),
            })
            .collect(),
        Some(fields) => fields
            .iter()
            .map(|f| {
                let known = columns.iter().find(|c| c.header == f || c.path == f);
                match known {
                    Some(c) => OwnedColumn {
                        header: c.header.to_string(),
                        path: c.path.to_string(),
                    },
                    None => OwnedColumn {
                        header: f.clone(),
                        path: f.clone(),
                    },
                }
            })
            .collect(),
    }
}

struct OwnedColumn {
    header: String,
    path: String,
}

/// Public variant of [`project`] for commands that render their own JSON
/// (e.g. `edesk api`).
pub fn project_fields(value: Value, fields: Option<&[String]>) -> Value {
    project(value, fields)
}

/// Project `--fields` into JSON output: keeps only the listed top-level keys
/// (dot-paths supported) on each object.
fn project(value: Value, fields: Option<&[String]>) -> Value {
    let Some(fields) = fields else { return value };
    match value {
        Value::Array(items) => Value::Array(
            items
                .into_iter()
                .map(|item| project_object(&item, fields))
                .collect(),
        ),
        other => project_object(&other, fields),
    }
}

fn project_object(value: &Value, fields: &[String]) -> Value {
    let mut map = serde_json::Map::new();
    for field in fields {
        map.insert(
            field.clone(),
            lookup(value, field).cloned().unwrap_or(Value::Null),
        );
    }
    Value::Object(map)
}

/// Resolve a dot-separated path (`user.name`) inside a JSON value.
fn lookup<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    for part in path.split('.') {
        current = match current {
            Value::Object(map) => map.get(part)?,
            Value::Array(items) => items.get(part.parse::<usize>().ok()?)?,
            _ => return None,
        };
    }
    Some(current)
}

fn cell_text(item: &Value, path: &str) -> String {
    lookup(item, path).map(display_value).unwrap_or_default()
}

/// Compact, human-friendly rendering of a JSON value for table cells.
fn display_value(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Array(items) => {
            let parts: Vec<String> = items.iter().map(display_value).collect();
            truncate(&parts.join(", "), 80)
        }
        Value::Object(_) => truncate(&value.to_string(), 80),
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let cut: String = s.chars().take(max.saturating_sub(1)).collect();
        format!("{cut}…")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn lookup_resolves_nested_paths() {
        let value = json!({"user": {"name": "Ana", "tags": [{"id": 7}]}});
        assert_eq!(lookup(&value, "user.name"), Some(&json!("Ana")));
        assert_eq!(lookup(&value, "user.tags.0.id"), Some(&json!(7)));
        assert_eq!(lookup(&value, "user.missing"), None);
    }

    #[test]
    fn project_keeps_only_requested_fields() {
        let value = json!([{"id": 1, "a": "x", "b": "y"}]);
        let fields = vec!["id".to_string(), "b".to_string()];
        assert_eq!(project(value, Some(&fields)), json!([{"id": 1, "b": "y"}]));
    }

    #[test]
    fn display_value_renders_scalars_and_collections() {
        assert_eq!(display_value(&json!(null)), "");
        assert_eq!(display_value(&json!(3)), "3");
        assert_eq!(display_value(&json!(true)), "true");
        assert_eq!(display_value(&json!(["a", "b"])), "a, b");
    }

    #[test]
    fn truncate_respects_char_boundaries() {
        assert_eq!(truncate("hola", 10), "hola");
        assert_eq!(truncate("aaaaaa", 3), "aa…");
        assert_eq!(truncate("ññññññ", 3), "ññ…");
    }

    #[test]
    fn fields_select_known_columns_by_header_or_path() {
        let columns = [col("order", "sales_order_id"), col("id", "id")];
        let fields = vec!["order".to_string(), "custom_key".to_string()];
        let selected = select_columns(&columns, Some(&fields));
        assert_eq!(selected[0].path, "sales_order_id");
        assert_eq!(selected[1].path, "custom_key");
    }
}
