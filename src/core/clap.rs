#[allow(unused)]
pub fn parse_json<T>(arg: &str) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    match serde_json::from_str(arg).map_err(|e| format!("Invalid JSON: {e}")) {
        Ok(val) => Ok(val),
        Err(e) => serde_json::from_str(&format!("\"{arg}\"")).map_err(|_| e),
    }
}
#[allow(unused)]
pub fn parse_upload_file(arg: &str) -> Result<super::upload_file::UploadFile, String> {
    super::upload_file::UploadFile::from_path(arg).map_err(|e| format!("{e}"))
}
#[allow(unused)]
pub fn parse_patch<T>(arg: &str) -> Result<super::patch::Patch<T>, String>
where
    T: serde::de::DeserializeOwned,
{
    if arg == "___undefined___" {
        Ok(super::patch::Patch::Undefined)
    } else if arg == "null" {
        Ok(super::patch::Patch::Null)
    } else {
        let parsed = parse_json::<T>(arg)?;
        Ok(super::patch::Patch::new(parsed))
    }
}
#[allow(unused)]
pub fn parse_patch_upload_file(
    arg: &str,
) -> Result<super::patch::Patch<super::upload_file::UploadFile>, String> {
    if arg == "___undefined___" {
        Ok(super::patch::Patch::Undefined)
    } else if arg == "null" {
        Ok(super::patch::Patch::Null)
    } else {
        let file = parse_upload_file(arg)?;
        Ok(super::patch::Patch::new(file))
    }
}
