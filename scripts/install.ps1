# Programmer Kit (pkit) Installation Script for Windows
# This script downloads the latest pkit executable from GitHub releases and installs it to ~/.pkit
# Can be run from anywhere - no build required

param(
    [string]$PKIT_REPO = "dead-projects-inc/pkit-cli",
    [string]$PKIT_VERSION = "latest"
)

$ErrorActionPreference = "Stop"

function Write-Status {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

if (-not (Get-Command curl -ErrorAction SilentlyContinue)) {
    Write-Error "curl is required but not installed."
    exit 1
}

$INSTALL_DIR = Join-Path $env:USERPROFILE ".pkit"
$BIN_DIR = Join-Path $INSTALL_DIR "bin"
$TEMP_DIR = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }

function Cleanup {
    if (Test-Path $TEMP_DIR) {
        Remove-Item $TEMP_DIR -Recurse -Force
    }
}

try {
    Write-Status "Installing pkit from $PKIT_REPO (version: $PKIT_VERSION)"

    $ARCH = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }
    $OS = "windows"
    
    $BINARY_NAME = "pkit-${OS}-${ARCH}.exe"
    Write-Status "Detected system: $OS $ARCH"

    if ($PKIT_VERSION -eq "latest") {
        Write-Status "Fetching latest release..."
        $RELEASE_URL = "https://api.github.com/repos/$PKIT_REPO/releases/latest"
        $response = Invoke-RestMethod -Uri $RELEASE_URL
        $asset = $response.assets | Where-Object { $_.name -like "*$BINARY_NAME*" } | Select-Object -First 1
        if ($asset) {
            $DOWNLOAD_URL = $asset.browser_download_url
        } else {
            Write-Error "Could not find download URL for $BINARY_NAME"
            Write-Error "Check if a release exists at: https://github.com/$PKIT_REPO/releases"
            exit 1
        }
    } else {
        $DOWNLOAD_URL = "https://github.com/$PKIT_REPO/releases/download/$PKIT_VERSION/$BINARY_NAME"
    }

    Write-Status "Download URL: $DOWNLOAD_URL"

    Write-Status "Downloading pkit..."
    $DOWNLOAD_PATH = Join-Path $TEMP_DIR "pkit.exe"
    try {
        Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile $DOWNLOAD_PATH -UseBasicParsing
    } catch {
        Write-Error "Failed to download pkit"
        exit 1
    }

    if (-not (Test-Path $DOWNLOAD_PATH)) {
        Write-Error "Downloaded file not found"
        exit 1
    }

    $BINARY_PATH = $DOWNLOAD_PATH

    $CURRENT_VERSION = ""
    $PKIT_EXE = Join-Path $BIN_DIR "pkit.exe"
    if (Test-Path $PKIT_EXE) {
        Write-Status "Found existing installation"
        try {
            $CURRENT_VERSION = & $PKIT_EXE --version 2>&1
            Write-Status "Current version: $CURRENT_VERSION"
        } catch {
            Write-Status "Could not determine current version"
        }
        
        if ($PKIT_VERSION -eq "latest") {
            $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$PKIT_REPO/releases/latest"
            $LATEST_VERSION = $response.tag_name
            if ($LATEST_VERSION -and $CURRENT_VERSION -like "*$LATEST_VERSION*") {
                Write-Success "Already up to date ($CURRENT_VERSION)"
                exit 0
            }
        }
    }

    Write-Status "Installing to $BIN_DIR"
    if (-not (Test-Path $BIN_DIR)) {
        New-Item -ItemType Directory -Path $BIN_DIR -Force | Out-Null
    }

    if (Test-Path $PKIT_EXE) {
        $BACKUP_PATH = Join-Path $BIN_DIR "pkit.exe.backup"
        Copy-Item $PKIT_EXE $BACKUP_PATH -ErrorAction SilentlyContinue
    }

    Copy-Item $BINARY_PATH $PKIT_EXE -Force

    try {
        & $PKIT_EXE --version | Out-Null
        if (Test-Path (Join-Path $BIN_DIR "pkit.exe.backup")) {
            Remove-Item (Join-Path $BIN_DIR "pkit.exe.backup") -ErrorAction SilentlyContinue
        }
    } catch {
        if (Test-Path (Join-Path $BIN_DIR "pkit.exe.backup")) {
            Move-Item (Join-Path $BIN_DIR "pkit.exe.backup") $PKIT_EXE -Force
            Write-Error "New binary failed, restored backup"
            exit 1
        }
        Write-Warning "Binary test failed"
    }

    function Setup-PowerShellProfile {
        param([string]$ProfilePath)
        
        $ProfileName = Split-Path $ProfilePath -Leaf
        
        if (-not (Test-Path $ProfilePath)) {
            New-Item -ItemType File -Path $ProfilePath -Force | Out-Null
            Write-Status "Created $ProfileName"
        }
        
        $BackupPath = "$ProfilePath.pkit-backup"
        Copy-Item $ProfilePath $BackupPath -ErrorAction SilentlyContinue
        
        $content = Get-Content $ProfilePath -ErrorAction SilentlyContinue
        if ($content) {
            $content = $content | Where-Object { $_ -notmatch "# pkit-cli-env-start" -and $_ -notmatch "# pkit-cli-env-end" -and $_ -notmatch "function pkit" -and $_ -notmatch "# Added by pkit installer" -and $_ -notmatch "\`$env:PATH.*\.pkit" }
            
            $inPkitBlock = $false
            $inPkitFunction = $false
            $filteredContent = @()
            
            foreach ($line in $content) {
                if ($line -match "# pkit-cli-env-start") {
                    $inPkitBlock = $true
                    continue
                }
                if ($line -match "# pkit-cli-env-end") {
                    $inPkitBlock = $false
                    continue
                }
                if ($line -match "function pkit") {
                    $inPkitFunction = $true
                    continue
                }
                if ($inPkitFunction -and $line -match "^\s*\}\s*$") {
                    $inPkitFunction = $false
                    continue
                }
                if (-not $inPkitBlock -and -not $inPkitFunction) {
                    $filteredContent += $line
                }
            }
            
            Set-Content $ProfilePath -Value $filteredContent -ErrorAction SilentlyContinue
        }
        
        $shellConfig = @"

# pkit-cli-env-start
`$env:PKIT_HOME = "`$env:USERPROFILE\.pkit"
`$env:PATH = "`$env:USERPROFILE\.pkit\bin;`$env:PATH"
if (Test-Path "`$env:PKIT_HOME\pkit_env.ps1") { . "`$env:PKIT_HOME\pkit_env.ps1" }
# pkit-cli-env-end

function pkit {
    & pkit.exe @args

    `$env_file = "`$(`$env:PKIT_HOME ?? "`$env:USERPROFILE\.pkit")\pkit_env.ps1"
    `$session_env_file = "`$(`$env:PKIT_HOME ?? "`$env:USERPROFILE\.pkit")\pkit_session_env.ps1"

    if (Test-Path `$env_file) {
        switch (`$args[0]) {
            { `$_ -in @("default", "install", "uninstall") } {
                . `$env_file
                Write-Host "pkit environment reloaded."
            }
            "switch" {
                . `$env_file
                if (Test-Path `$session_env_file) {
                    . `$session_env_file
                    Write-Host "pkit session environment loaded."
                }
            }
        }
    } elseif (`$args[0] -in @("default", "install", "uninstall", "switch")) {
        Write-Warning "Environment file not found at `$env_file"
    }
}
"@
        
        Add-Content $ProfilePath -Value $shellConfig
        Write-Success "Configured $ProfileName"
    }

    Write-Status "Configuring PowerShell environment..."
    
    $profiles = @(
        $PROFILE.CurrentUserCurrentHost,
        $PROFILE.CurrentUserAllHosts
    )
    
    foreach ($profile in $profiles) {
        if ($profile) {
            Setup-PowerShellProfile $profile
        }
    }

    $uninstallScript = @"
# Uninstall script for pkit (Windows)

Write-Host "Uninstalling pkit..." -ForegroundColor Yellow

`$profiles = @(
    `$PROFILE.CurrentUserCurrentHost,
    `$PROFILE.CurrentUserAllHosts
)

foreach (`$profile in `$profiles) {
    if (`$profile -and (Test-Path `$profile)) {
        `$content = Get-Content `$profile -ErrorAction SilentlyContinue
        if (`$content) {
            `$inPkitBlock = `$false
            `$inPkitFunction = `$false
            `$filteredContent = @()
            
            foreach (`$line in `$content) {
                if (`$line -match "# pkit-cli-env-start") {
                    `$inPkitBlock = `$true
                    continue
                }
                if (`$line -match "# pkit-cli-env-end") {
                    `$inPkitBlock = `$false
                    continue
                }
                if (`$line -match "function pkit") {
                    `$inPkitFunction = `$true
                    continue
                }
                if (`$inPkitFunction -and `$line -match "^\s*\}\s*$") {
                    `$inPkitFunction = `$false
                    continue
                }
                if (-not `$inPkitBlock -and -not `$inPkitFunction) {
                    `$filteredContent += `$line
                }
            }
            
            Set-Content `$profile -Value `$filteredContent -ErrorAction SilentlyContinue
            Write-Host "Cleaned `$(Split-Path `$profile -Leaf)" -ForegroundColor Green
        }
    }
}

if (Test-Path "`$env:USERPROFILE\.pkit") {
    Remove-Item "`$env:USERPROFILE\.pkit" -Recurse -Force
    Write-Host "Removed ~/.pkit" -ForegroundColor Green
}

Write-Host "Uninstall complete" -ForegroundColor Green
Write-Host "Please restart your PowerShell session" -ForegroundColor Yellow
"@

    $uninstallPath = Join-Path $INSTALL_DIR "uninstall.ps1"
    Set-Content $uninstallPath -Value $uninstallScript

    try {
        $VERSION_OUTPUT = & $PKIT_EXE --version 2>&1
    } catch {
        $VERSION_OUTPUT = "unknown"
    }

    if ($CURRENT_VERSION) {
        Write-Success "Updated pkit successfully!"
        Write-Status "Previous: $CURRENT_VERSION"
        Write-Status "Current: $VERSION_OUTPUT"
    } else {
        Write-Success "Installed pkit successfully!"
        Write-Status "Version: $VERSION_OUTPUT"
    }

    Write-Status "Location: $PKIT_EXE"
    Write-Status "Uninstall: $uninstallPath"
    Write-Warning "Restart your PowerShell session or run: . `$PROFILE"
    Write-Status "Test with: pkit --help"

} finally {
    Cleanup
}
