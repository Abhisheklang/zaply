use std::fs;
use zaply::settings::LspSettings;
use zaply_extension_api::{self as zaply, LanguageServerId, Result, serde_json};

struct GlslExtension {
    cached_binary_path: Option<String>,
}

impl GlslExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zaply::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("glsl_analyzer") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path
            && fs::metadata(path).is_ok_and(|stat| stat.is_file())
        {
            return Ok(path.clone());
        }

        zaply::set_language_server_installation_status(
            language_server_id,
            &zaply::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zaply::latest_github_release(
            "nolanderc/glsl_analyzer",
            zaply::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zaply::current_platform();
        let asset_name = format!(
            "{arch}-{os}.zip",
            arch = match arch {
                zaply::Architecture::Aarch64 => "aarch64",
                zaply::Architecture::X86 => "x86",
                zaply::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zaply::Os::Mac => "macos",
                zaply::Os::Linux => "linux-musl",
                zaply::Os::Windows => "windows",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("glsl_analyzer-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|err| format!("failed to create directory '{version_dir}': {err}"))?;
        let binary_path = format!("{version_dir}/bin/glsl_analyzer");

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zaply::set_language_server_installation_status(
                language_server_id,
                &zaply::LanguageServerInstallationStatus::Downloading,
            );

            zaply::download_file(
                &asset.download_url,
                &version_dir,
                match platform {
                    zaply::Os::Mac | zaply::Os::Linux => zaply::DownloadedFileType::Zip,
                    zaply::Os::Windows => zaply::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zaply::make_file_executable(&binary_path)?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zaply::Extension for GlslExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zaply::LanguageServerId,
        worktree: &zaply::Worktree,
    ) -> Result<zaply::Command> {
        Ok(zaply::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zaply::LanguageServerId,
        worktree: &zaply::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("glsl_analyzer", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings)
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            "glsl_analyzer": settings
        })))
    }
}

zaply::register_extension!(GlslExtension);
