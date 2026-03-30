use zed_extension_api as zed;

struct SkyExtension;

impl zed::Extension for SkyExtension {
    fn new() -> Self {
        SkyExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let sky_path = worktree
            .which("sky")
            .ok_or_else(|| "Could not find `sky` binary in PATH. Install sky first.".to_string())?;

        Ok(zed::Command {
            command: sky_path,
            args: vec!["lsp".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(SkyExtension);
