use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};

pub(crate) const SPLIT_LOADER_STEM: &str = "__wasm_split";
pub(crate) const SPLIT_MANIFEST_STEM: &str = "__wasm_split_manifest";

#[derive(Debug, Clone)]
pub(crate) struct WasmSplitStageOutput {
    pub(crate) main_wasm_path: PathBuf,
    pub(crate) split_loader_output: String,
    pub(crate) split_manifest_output: String,
    pub(crate) split_wasm_outputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct WasmSplitManifest {
    pub(crate) loader: String,
    pub(crate) prefetch_map: BTreeMap<String, Vec<String>>,
}

pub(crate) fn hashed_file_name(stem: &str, ext: &str, bundle_hash: Option<&str>) -> String {
    match bundle_hash {
        Some(bundle_hash) => format!("{stem}-{bundle_hash}.{ext}"),
        None => format!("{stem}.{ext}"),
    }
}

pub(crate) fn split_loader_file_name(bundle_hash: Option<&str>) -> String {
    hashed_file_name(SPLIT_LOADER_STEM, "js", bundle_hash)
}

pub(crate) fn split_manifest_file_name(bundle_hash: Option<&str>) -> String {
    hashed_file_name(SPLIT_MANIFEST_STEM, "json", bundle_hash)
}

pub(crate) fn split_wasm_file_name(path: &Path, bundle_hash: Option<&str>) -> Result<String> {
    let stem = path
        .file_stem()
        .context("wasm-split module is missing a file stem")?
        .to_string_lossy();
    Ok(hashed_file_name(&stem, "wasm", bundle_hash))
}

pub(crate) fn rewrite_loader_paths(
    mut source: String,
    chunk_renames: &HashMap<String, String>,
) -> String {
    let mut renames: Vec<_> = chunk_renames.iter().collect();
    renames.sort_by_key(|(from, _)| std::cmp::Reverse(from.len()));

    for (from, to) in renames {
        source = source.replace(&format!("./{from}"), &format!("./{to}"));
    }

    source
}

pub(crate) fn rewrite_prefetch_map(
    prefetch_map: HashMap<String, Vec<String>>,
    file_renames: &HashMap<String, String>,
) -> Result<BTreeMap<String, Vec<String>>> {
    prefetch_map
        .into_iter()
        .map(|(split, files)| {
            let files = files
                .into_iter()
                .map(|file| {
                    file_renames.get(&file).cloned().with_context(|| {
                        format!(
                            "missing hashed split output for prefetch map entry '{split}' -> '{file}'"
                        )
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            Ok((split, files))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        hashed_file_name, rewrite_loader_paths, rewrite_prefetch_map, split_loader_file_name,
        split_manifest_file_name, WasmSplitManifest,
    };
    use anyhow::Result;
    use std::collections::{BTreeMap, HashMap};

    #[test]
    fn hashes_split_loader_name() {
        assert_eq!(
            split_loader_file_name(Some("abc123")),
            "__wasm_split-abc123.js"
        );
        assert_eq!(split_loader_file_name(None), "__wasm_split.js");
    }

    #[test]
    fn hashes_split_manifest_name() {
        assert_eq!(
            split_manifest_file_name(Some("abc123")),
            "__wasm_split_manifest-abc123.json"
        );
        assert_eq!(split_manifest_file_name(None), "__wasm_split_manifest.json");
    }

    #[test]
    fn hashes_chunk_name() {
        assert_eq!(
            hashed_file_name("chunk_0", "wasm", Some("abc123")),
            "chunk_0-abc123.wasm"
        );
        assert_eq!(hashed_file_name("chunk_0", "wasm", None), "chunk_0.wasm");
    }

    #[test]
    fn rewrites_split_loader_chunk_paths() {
        let source = concat!(
            "const a = new URL(\"./chunk_0.wasm\", import.meta.url);\n",
            "const b = new URL(\"./split_route.wasm\", import.meta.url);\n",
        )
        .to_string();
        let renames = HashMap::from([
            (
                "chunk_0.wasm".to_string(),
                "chunk_0-deadbeef.wasm".to_string(),
            ),
            (
                "split_route.wasm".to_string(),
                "split_route-deadbeef.wasm".to_string(),
            ),
        ]);

        let rewritten = rewrite_loader_paths(source, &renames);

        assert!(rewritten.contains("./chunk_0-deadbeef.wasm"));
        assert!(rewritten.contains("./split_route-deadbeef.wasm"));
        assert!(!rewritten.contains("./chunk_0.wasm"));
        assert!(!rewritten.contains("./split_route.wasm"));
    }

    #[test]
    fn rewrites_prefetch_map_with_hashed_filenames() -> Result<()> {
        let prefetch_map = HashMap::from([
            (
                "load_greeting".to_string(),
                vec!["chunk_0".to_string(), "split_load_greeting".to_string()],
            ),
            ("route".to_string(), vec!["split_route".to_string()]),
        ]);
        let renames = HashMap::from([
            ("chunk_0".to_string(), "chunk_0-deadbeef.wasm".to_string()),
            (
                "split_load_greeting".to_string(),
                "split_load_greeting-deadbeef.wasm".to_string(),
            ),
            (
                "split_route".to_string(),
                "split_route-deadbeef.wasm".to_string(),
            ),
        ]);

        let rewritten = rewrite_prefetch_map(prefetch_map, &renames)?;

        assert_eq!(
            rewritten,
            BTreeMap::from([
                (
                    "load_greeting".to_string(),
                    vec![
                        "chunk_0-deadbeef.wasm".to_string(),
                        "split_load_greeting-deadbeef.wasm".to_string(),
                    ],
                ),
                (
                    "route".to_string(),
                    vec!["split_route-deadbeef.wasm".to_string()],
                ),
            ])
        );

        Ok(())
    }

    #[test]
    fn manifest_serializes_loader_and_prefetch_map() -> Result<()> {
        let manifest = WasmSplitManifest {
            loader: "__wasm_split-deadbeef.js".to_string(),
            prefetch_map: BTreeMap::from([(
                "route".to_string(),
                vec!["split_route-deadbeef.wasm".to_string()],
            )]),
        };

        let json = serde_json::to_value(&manifest)?;

        assert_eq!(json["loader"], "__wasm_split-deadbeef.js");
        assert_eq!(
            json["prefetch_map"]["route"][0],
            "split_route-deadbeef.wasm"
        );

        Ok(())
    }
}
