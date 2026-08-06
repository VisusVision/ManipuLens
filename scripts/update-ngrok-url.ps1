<#
.SYNOPSIS
    extension/server_config.json içindeki ngrok_url'i yerel ngrok tünelinden okunan
    güncel adresle senkronize eder (elle "Update server_config.json" commit'lerinin yerine).

.PARAMETER Commit
    Değişikliği otomatik commit'ler. Push YAPMAZ — push elle yapılır.

.EXAMPLE
    ./scripts/update-ngrok-url.ps1
    ./scripts/update-ngrok-url.ps1 -Commit
#>
param(
    [switch]$Commit
)

$ErrorActionPreference = "Stop"
$repoRoot = Split-Path -Parent $PSScriptRoot
$configPath = Join-Path $repoRoot "extension\server_config.json"

try {
    $tunnels = Invoke-RestMethod -Uri "http://127.0.0.1:4040/api/tunnels" -TimeoutSec 5
} catch {
    Write-Error "ngrok yerel API'sine ulaşılamadı (http://127.0.0.1:4040). ngrok çalışıyor mu?"
    exit 1
}

$httpsTunnel = $tunnels.tunnels | Where-Object { $_.public_url -like "https://*" } | Select-Object -First 1
if (-not $httpsTunnel) {
    Write-Error "Aktif https ngrok tüneli bulunamadı."
    exit 1
}

$newUrl = $httpsTunnel.public_url
$current = Get-Content $configPath -Raw | ConvertFrom-Json

if ($current.ngrok_url -eq $newUrl) {
    Write-Output "ngrok_url zaten güncel: $newUrl"
    exit 0
}

Write-Output "ngrok_url güncelleniyor: $($current.ngrok_url) -> $newUrl"
$updated = [ordered]@{ ngrok_url = $newUrl }
($updated | ConvertTo-Json) | Set-Content -Path $configPath -Encoding utf8

if ($Commit) {
    git -C $repoRoot add $configPath
    git -C $repoRoot commit -m "chore: ngrok_url guncelle ($newUrl)"
    Write-Output "Commit alindi. Push icin: git push"
}
