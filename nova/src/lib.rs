use mlua::{Lua, Function, Error as LuaError};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub enum NovaError {
    Io(io::Error),
    Lua(LuaError),
    MissingInstallFunction,
}

impl From<io::Error> for NovaError {
    fn from(e: io::Error) -> Self {
        NovaError::Io(e)
    }
}

impl From<LuaError> for NovaError {
    fn from(e: LuaError) -> Self {
        NovaError::Lua(e)
    }
}

fn within_root(root: &Path, requested: &str) -> PathBuf {
    let raw = Path::new(requested);
    if raw.is_absolute() {
        root.join(raw.strip_prefix("/").unwrap_or(raw))
    } else {
        root.join(raw)
    }
}

pub fn run_nova_build_script(script_path: &str, extraction_root: &Path, install_root: &Path) -> Result<(), NovaError> {
    let lua = Lua::new();
    let code = fs::read_to_string(script_path)?;
    let install_root_buf = install_root.to_path_buf();
    let extraction_root_buf = extraction_root.to_path_buf();

    let globals = lua.globals();
    globals.set("install_root", install_root.display().to_string())?;

    let run = {
        let extract_root = extraction_root_buf.clone();
        lua.create_function(move |_, args: Vec<String>| {
            if args.is_empty() {
                return Err(LuaError::external("run() requires at least one argument"));
            }
            let mut cmd = Command::new(&args[0]);
            if args.len() > 1 {
                cmd.args(&args[1..]);
            }
            cmd.current_dir(&extract_root);
            let status = cmd.status()?;
            if !status.success() {
                return Err(LuaError::external(format!("Command failed: {:?}", args)));
            }
            Ok(status.code().unwrap_or(1))
        })?
    };

    let copy = {
        let install_root = install_root_buf.clone();
        let extract_root = extraction_root_buf.clone();
        lua.create_function_mut(move |_, (from, to): (String, String)| {
            let full_from = extract_root.join("files").join(&from);
            let full_to = within_root(&install_root, &to);
            fs::create_dir_all(full_to.parent().unwrap_or_else(|| Path::new("/")))?;
            fs::copy(&full_from, &full_to)?;
            Ok(())
        })?
    };

    let symlink = {
        let install_root = install_root_buf.clone();
        lua.create_function(move |_, (target, linkname): (String, String)| {
            let full_link = within_root(&install_root, &linkname);
            let _ = fs::remove_file(&full_link);
            std::os::unix::fs::symlink(&target, &full_link)?;
            Ok(())
        })?
    };

    let mkdir = {
        let install_root = install_root_buf.clone();
        lua.create_function(move |_, path: String| {
            let full_path = within_root(&install_root, &path);
            fs::create_dir_all(&full_path)?;
            Ok(())
        })?
    };

    let chmod = {
        let install_root = install_root_buf.clone();
        lua.create_function(move |_, (path, mode): (String, u32)| {
            let full_path = within_root(&install_root, &path);
            let mut perms = fs::metadata(&full_path)?.permissions();
            perms.set_mode(mode);
            fs::set_permissions(&full_path, perms)?;
            Ok(())
        })?
    };

    let exists = {
        let install_root = install_root_buf.clone();
        lua.create_function(move |_, path: String| {
            let full_path = within_root(&install_root, &path);
            Ok(full_path.exists())
        })?
    };

    globals.set("run", run)?;
    globals.set("copy", copy)?;
    globals.set("symlink", symlink)?;
    globals.set("mkdir", mkdir)?;
    globals.set("chmod", chmod)?;
    globals.set("exists", exists)?;

    lua.load(&code).exec()?;

    match globals.get::<Function>("build") {
        Ok(build_fn) => {
            build_fn.call(())?;
            Ok(())
        }
        Err(LuaError::FromLuaConversionError { .. }) => {
            println!("⚠️  No build() defined in script. Skipping.");
            Ok(())
        }
        Err(e) => Err(NovaError::Lua(e)),
    }
}

pub fn run_nova_script(
    script_path: &str,
    extraction_root: &Path,
    install_root: &Path,
    installed_files: &mut Vec<String>,
) -> Result<(), NovaError> {
    let lua = Lua::new();
    let code = fs::read_to_string(script_path)?;
    let install_root_buf = install_root.to_path_buf();
    let extraction_root_buf = extraction_root.to_path_buf();

    let installed = Rc::new(RefCell::new(Vec::new()));

    let globals = lua.globals();
    globals.set("install_root", install_root_buf.display().to_string())?;

    let run = {
        let extract_root = extraction_root_buf.clone();
        lua.create_function_mut(move |_, args: Vec<String>| {
            if args.is_empty() {
                return Err(LuaError::external("run() requires at least one argument"));
            }
            let mut cmd = Command::new(&args[0]);
            if args.len() > 1 {
                cmd.args(&args[1..]);
            }
            cmd.current_dir(&extract_root);
            let status = cmd.status()?;
            if !status.success() {
                return Err(LuaError::external(format!("Command failed: {:?}", args)));
            }
            Ok(status.code().unwrap_or(1))
        })?
    };

    let copy = {
        let install_root = install_root_buf.clone();
        let extract_root = extraction_root_buf.clone();
        let installed_files = Rc::clone(&installed);

        lua.create_function_mut(move |_, (from, to): (String, String)| {
            let full_from = extract_root.join("files").join(&from);
            let full_to = within_root(&install_root, &to);

            if !full_from.exists() {
                return Err(LuaError::external(format!("Source file does not exist: {}", full_from.display())));
            }

            fs::create_dir_all(full_to.parent().unwrap_or_else(|| Path::new("/")))?;
            fs::copy(&full_from, &full_to)?;

            let recorded_path = format!("/{}", to.trim_start_matches("./").trim_start_matches('/'));
            installed_files.borrow_mut().push(recorded_path);

            Ok(())
        })?
    };

    let symlink = {
        let install_root = install_root_buf.clone();
        lua.create_function_mut(move |_, (target, linkname): (String, String)| {
            let full_link = within_root(&install_root, &linkname);
            let _ = fs::remove_file(&full_link);
            std::os::unix::fs::symlink(&target, &full_link)?;
            Ok(())
        })?
    };

    let mkdir = {
        let install_root = install_root_buf.clone();
        lua.create_function_mut(move |_, path: String| {
            let full_path = within_root(&install_root, &path);
            fs::create_dir_all(&full_path)?;
            Ok(())
        })?
    };

    let chmod = {
        let install_root = install_root_buf.clone();
        lua.create_function_mut(move |_, (path, mode): (String, u32)| {
            let full_path = within_root(&install_root, &path);
            let mut perms = fs::metadata(&full_path)?.permissions();
            perms.set_mode(mode);
            fs::set_permissions(&full_path, perms)?;
            Ok(())
        })?
    };

    let exists = {
        let install_root = install_root_buf.clone();
        lua.create_function_mut(move |_, path: String| {
            let full_path = within_root(&install_root, &path);
            Ok(full_path.exists())
        })?
    };

    globals.set("run", run)?;
    globals.set("copy", copy)?;
    globals.set("symlink", symlink)?;
    globals.set("mkdir", mkdir)?;
    globals.set("chmod", chmod)?;
    globals.set("exists", exists)?;

    lua.load(&code).exec()?;

    match globals.get::<Function>("install") {
        Ok(install_fn) => {
            install_fn.call(())?;
            installed_files.extend(installed.borrow().clone());
            Ok(())
        }
        Err(_) => Err(NovaError::MissingInstallFunction),
    }
}