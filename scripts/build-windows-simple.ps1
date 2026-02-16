# Simple Windows installer builder (creates ZIP package)
# No external dependencies required

$ErrorActionPreference = "Stop"

$ProjectRoot = Split-Path -Parent $PSScriptRoot
$Version = (Select-String -Path "$ProjectRoot\Cargo.toml" -Pattern '^version\s*=\s*"([^"]+)"').Matches[0].Groups[1].Value
$BuildDir = "$ProjectRoot\build\windows-simple"
$DistDir = "$ProjectRoot\dist"

Write-Host "Building KeySwitch Windows package v$Version..." -ForegroundColor Green

# Clean and create build directories
if (Test-Path $BuildDir) {
    Remove-Item -Recurse -Force $BuildDir
}
New-Item -ItemType Directory -Force -Path "$BuildDir\KeySwitch" | Out-Null
New-Item -ItemType Directory -Force -Path $DistDir | Out-Null

# Build the release binary
Write-Host "Building release binary..." -ForegroundColor Cyan
Set-Location $ProjectRoot
cargo build --release

# Copy binary
Write-Host "Copying files..." -ForegroundColor Cyan
Copy-Item "$ProjectRoot\target\release\keyswitch.exe" "$BuildDir\KeySwitch\keyswitch.exe"

# Create installation script
$InstallScript = @"
@echo off
setlocal enabledelayedexpansion

echo.
echo ========================================
echo KeySwitch Installer
echo ========================================
echo.

REM Check for admin rights
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: This installer requires administrator privileges.
    echo Please right-click and select "Run as administrator"
    pause
    exit /b 1
)

REM Set installation directory
set INSTALL_DIR=%ProgramFiles%\KeySwitch

echo Installing to: %INSTALL_DIR%
echo.

REM Create directory
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

REM Copy executable
echo Copying files...
copy /Y keyswitch.exe "%INSTALL_DIR%\keyswitch.exe" >nul

REM Create startup shortcut
echo Setting up auto-start...
set STARTUP_DIR=%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup
powershell -Command "`$ws = New-Object -ComObject WScript.Shell; `$s = `$ws.CreateShortcut('%STARTUP_DIR%\KeySwitch.lnk'); `$s.TargetPath = '%INSTALL_DIR%\keyswitch.exe'; `$s.Arguments = '--daemon'; `$s.WindowStyle = 7; `$s.Save()"

REM Start the daemon
echo Starting KeySwitch daemon...
start "" "%INSTALL_DIR%\keyswitch.exe" --daemon

echo.
echo ========================================
echo Installation Complete!
echo ========================================
echo.
echo KeySwitch has been installed and started.
echo It will automatically start when you log in.
echo.
echo Installation directory: %INSTALL_DIR%
echo Logs: %TEMP%\keyswitch.log
echo.
echo To uninstall, run: uninstall.bat
echo.
pause
"@

$InstallScript | Out-File -FilePath "$BuildDir\KeySwitch\install.bat" -Encoding ASCII

# Create uninstallation script
$UninstallScript = @"
@echo off
setlocal enabledelayedexpansion

echo.
echo ========================================
echo KeySwitch Uninstaller
echo ========================================
echo.

REM Check for admin rights
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: This uninstaller requires administrator privileges.
    echo Please right-click and select "Run as administrator"
    pause
    exit /b 1
)

set INSTALL_DIR=%ProgramFiles%\KeySwitch
set STARTUP_DIR=%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup

echo Stopping KeySwitch daemon...
taskkill /F /IM keyswitch.exe >nul 2>&1

echo Removing auto-start...
if exist "%STARTUP_DIR%\KeySwitch.lnk" del "%STARTUP_DIR%\KeySwitch.lnk"

echo Removing files...
if exist "%INSTALL_DIR%\keyswitch.exe" del "%INSTALL_DIR%\keyswitch.exe"
if exist "%INSTALL_DIR%\uninstall.bat" del "%INSTALL_DIR%\uninstall.bat"
if exist "%INSTALL_DIR%" rmdir "%INSTALL_DIR%" 2>nul

echo.
echo ========================================
echo Uninstallation Complete!
echo ========================================
echo.
pause
"@

$UninstallScript | Out-File -FilePath "$BuildDir\KeySwitch\uninstall.bat" -Encoding ASCII

# Create README
$Readme = @"
KeySwitch v$Version
==================

INSTALLATION
------------
1. Right-click on install.bat
2. Select "Run as administrator"
3. Follow the prompts

The daemon will start automatically and run on startup.

UNINSTALLATION
--------------
1. Right-click on uninstall.bat (from the installation directory)
2. Select "Run as administrator"

MANUAL OPERATION
----------------
To run manually:
    keyswitch.exe --daemon

To check if it's running:
    tasklist | findstr keyswitch

To view logs:
    type %TEMP%\keyswitch.log

WHAT IT DOES
------------
KeySwitch runs in the background and automatically sets your Keychron
keyboard to Windows mode when it connects (e.g., after a KVM switch).
"@

$Readme | Out-File -FilePath "$BuildDir\KeySwitch\README.txt" -Encoding ASCII

# Create ZIP package
Write-Host "Creating ZIP package..." -ForegroundColor Cyan
$ZipPath = "$DistDir\KeySwitch-${Version}-Windows.zip"
Compress-Archive -Path "$BuildDir\KeySwitch\*" -DestinationPath $ZipPath -Force

Write-Host ""
Write-Host "✅ Package created successfully!" -ForegroundColor Green
Write-Host "📦 Location: dist\KeySwitch-${Version}-Windows.zip" -ForegroundColor Cyan
Write-Host ""
Write-Host "Users should:" -ForegroundColor Yellow
Write-Host "  1. Extract the ZIP file" -ForegroundColor Yellow
Write-Host "  2. Right-click install.bat" -ForegroundColor Yellow
Write-Host "  3. Select 'Run as administrator'" -ForegroundColor Yellow
