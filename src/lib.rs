use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        version
    } else {
        format!("@{}", version)
    };

    let path = dag().get_env("PATH")?;
    let home = dag().get_env("HOME")?;
    dag().set_envs(vec![(
        "PATH".into(),
        format!(
            "{}/.local/bin:{}/.local/share/mise/shims:{}",
            home, home, path
        ),
    )])?;

    let stdout = dag()
        .mise()?
        .with_exec(vec!["mise", "install", &format!("java@{}", version)])?
        .with_exec(vec!["mise", "use", "-g", "java"])?
        .stdout()?;
    Ok(stdout)
}
