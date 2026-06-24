use std::{env, io};
use winreg::{HKCU, RegKey, enums::KEY_SET_VALUE};

type IoResult<T> = Result<T, io::Error>;

fn main() -> IoResult<()> {
    let mut args = env::args().skip(1);
    match (&args.next(),&args.next()) {
        (Some(name), value) => {
            if let Some(value) = value {
                set_var(name, value)?
            }else {
                remove_var(name)?
            }
        },
        _ => todo!(),
    }
    Ok(())
}
fn set_var(name: &str, value: &str) -> IoResult<()> {
    let cu_env = HKCU.open_subkey_with_flags(ENVIRONMENT, KEY_SET_VALUE)?;
    cu_env.set_value(name, &value)?;
    unsafe { env::set_var(name, value) };
    Ok(())
}
fn remove_var(name: &str) -> IoResult<()> {
    let cu_env = HKCU.open_subkey_with_flags(ENVIRONMENT, KEY_SET_VALUE)?;
    cu_env.delete_value(name)?;
    unsafe { env::remove_var(name) };
    Ok(())
}
fn path_vec(cu_env: &RegKey) -> Vec<String> {
    let path: String = cu_env.get_value(PATH).unwrap();
    let path: Vec<_> = path
        .split(";")
        .filter_map(|p| (!p.is_empty()).then_some(p.to_string()))
        .collect();
    path
}
const ENVIRONMENT: &str = "Environment";
const PATH: &str = "Path";
// struct Env {
//     name: String,
//     value: String,
// }
// fn env_iter(cu_env: &RegKey) -> impl Iterator<Item = Env> {
//     cu_env.enum_values().filter_map(|v| v.ok()).map(|v| Env {
//         name: v.0,
//         value: format!("{}", v.1),
//     })
// }
