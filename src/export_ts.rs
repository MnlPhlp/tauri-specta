use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use specta::{to_ts, to_ts_export, TypeDefs};

use crate::CommandDataType;

pub fn export_to_ts(
    (cmds, type_map): (Vec<CommandDataType>, TypeDefs),
    export_path: impl AsRef<Path>,
) -> Result<(), io::Error> {
    let export_path = PathBuf::from(export_path.as_ref());
    if let Some(export_dir) = export_path.parent() {
        fs::create_dir_all(export_dir)?;
    }
    let mut file = File::create(export_path)?;
    // if let Some(header) = &self.config.bindings_header {
    //     writeln!(file, "{}", header)?;
    // }
    writeln!(file, "// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.")?;
    writeln!(file, "")?;

    writeln!(
        file,
        "export type Commands ={};",
        cmds.into_iter()
            .map(|v| format!(
                r#" | {{ name: "{}", input: {}, result: {} }}"#,
                v.name,
                v.input.map(|v| to_ts(&v)).unwrap_or("null".into()),
                to_ts(&v.result)
            ))
            .collect::<String>()
    )?;

    for export in type_map.values().filter_map(|v| to_ts_export(v).ok()) {
        writeln!(file, "\n{}", export)?;
    }

    Ok(())
}
