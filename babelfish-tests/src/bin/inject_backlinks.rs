use std::fs;
use std::path::Path;

/// Insert `// Report: test-report/<slug>.md` immediately above the `#[test]`
/// attribute (or the `fn` line if no attribute) of the test enclosing 1-based
/// `line`. Idempotent: an existing `// Report:` line there is replaced. Returns
/// `None` when `line` is not inside a test fn (nothing edited).
fn inject(src: &str, line: u32, slug: &str) -> Option<String> {
    let lines: Vec<&str> = src.lines().collect();
    let idx = (line as usize).checked_sub(1)?;
    if idx >= lines.len() {
        return None;
    }
    // Scan up to the `fn` line of the enclosing item.
    let mut fn_idx = idx;
    while !lines[fn_idx].trim_start().starts_with("fn ") && !lines[fn_idx].contains(" fn ") {
        if fn_idx == 0 {
            return None;
        }
        fn_idx -= 1;
    }
    // Then up past attributes (`#[...]`) to the insertion point.
    let mut at = fn_idx;
    while at > 0 && lines[at - 1].trim_start().starts_with('#') {
        at -= 1;
    }
    let comment = format!("// Report: test-report/{slug}.md");
    let mut out: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    // Idempotence: replace an existing back-link directly above `at`.
    if at > 0 && out[at - 1].trim_start().starts_with("// Report:") {
        out[at - 1] = comment;
    } else {
        out.insert(at, comment);
    }
    let mut joined = out.join("\n");
    if src.ends_with('\n') {
        joined.push('\n');
    }
    Some(joined)
}

fn main() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let floors = Path::new(manifest).join("test-report/.floors");
    let entries = match fs::read_dir(&floors) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("no floors at {}; run `make test-report` first", floors.display());
            std::process::exit(1);
        }
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        let slug = json["slug"].as_str().unwrap();
        let prov = &json["provenance"];
        if prov.is_null() {
            eprintln!("floor {slug} has no provenance; skipping");
            continue;
        }
        let file = prov["file"].as_str().unwrap();
        let line = prov["line"].as_u64().unwrap() as u32;
        let src_path = Path::new(manifest).join(file);
        let src = fs::read_to_string(&src_path).unwrap();
        match inject(&src, line, slug) {
            Some(updated) => {
                fs::write(&src_path, updated).unwrap();
                println!("linked {file} -> test-report/{slug}.md");
            }
            None => eprintln!("could not locate a test fn at {file}:{line}; skipping {slug}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::inject;

    const SRC: &str = "\
use frood::Story;

#[test]
fn vested_tokens_are_forfeited() {
    let x = 1;
}
";

    #[test]
    fn inserts_backlink_above_the_test_attr() {
        // line 6 (1-based) is `let x = 1;`, inside the fn.
        let out = inject(SRC, 6, "vested-tokens-forfeited").unwrap();
        assert!(out.contains("// Report: test-report/vested-tokens-forfeited.md\n#[test]"), "{out}");
    }

    #[test]
    fn is_idempotent() {
        let once = inject(SRC, 6, "vested-tokens-forfeited").unwrap();
        let twice = inject(&once, 7, "vested-tokens-forfeited").unwrap();
        assert_eq!(once, twice, "second run must not duplicate the back-link");
    }

    #[test]
    fn skips_span_outside_any_test() {
        // line 2 is the `use` line, no enclosing #[test].
        assert!(inject(SRC, 2, "x").is_none());
    }
}
