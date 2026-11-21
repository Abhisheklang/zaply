
function ParseZaplyWorkspace {
    $metadata = cargo metadata --no-deps --offline | ConvertFrom-Json
    $env:ZAPLY_WORKSPACE = $metadata.workspace_root
    $env:RELEASE_VERSION = $metadata.packages | Where-Object { $_.name -eq "zaply" } | Select-Object -ExpandProperty version
}
