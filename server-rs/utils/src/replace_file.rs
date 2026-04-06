use std::env;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::chown;
use std::path::Path;

/// On Unix a running executable can be safely deleted.
pub fn self_delete(exe: &Path) -> Result<(), io::Error> {
    let exe = exe.canonicalize()?;
    fs::remove_file(exe)?;
    Ok(())
}

pub fn self_replace(new_executable: &Path) -> Result<(), io::Error> {
    let mut exe = env::current_exe()?;
    if fs::symlink_metadata(&exe).is_ok_and(|x| x.file_type().is_symlink()) {
        exe = fs::read_link(exe)?;
    }
    let old_permissions = exe.metadata()?.permissions();
    let old_uid = exe.metadata()?.uid();
    let old_gid = exe.metadata()?.gid();

    let prefix = if let Some(hint) = exe.file_stem().and_then(|x| x.to_str()) {
        format!(".{hint}.__temp__")
    } else {
        ".__temp__".into()
    };

    let tmp = tempfile::Builder::new().prefix(&prefix).tempfile_in(
        exe.parent()
            .ok_or_else(|| io::Error::other("executable has no known parent folder"))?,
    )?;
    fs::copy(new_executable, tmp.path())?;
    fs::set_permissions(tmp.path(), old_permissions)?;
    chown(tmp.path(), Some(old_uid), Some(old_gid))?;

    // if we made it this far, try to persist the temporary file and move it over.
    let (_, path) = tmp.keep()?;
    match fs::rename(&path, &exe) {
        Ok(()) => {}
        Err(err) => {
            fs::remove_file(&path).ok();
            return Err(err);
        }
    }

    Ok(())
}
