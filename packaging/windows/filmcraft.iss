; FilmCraft Inno Setup Script
; Windows x64 Installer with Thai and English support

#define MyAppName "FilmCraft"
#define MyAppVersion "0.7.2"
#define MyAppPublisher "CloudDev3344 (Original: Storytold / ArtCraft Team)"
#define MyAppURL "https://github.com/CloudDev3344/FilmCraft-"
#define MyAppOriginalURL "https://github.com/storytold/filmcraft"
#define MyAppExeName "filmcraft.exe"
#define MyAppCliName "filmcraft-cli.exe"

[Setup]
AppId={{D94B8F63-44E2-4C1B-82A1-03BD586A7A90}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}/releases
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
LicenseFile=..\..\LICENSE-MIT
OutputDir=..\..\dist\release
OutputBaseFilename=FilmCraft-0.7.2-Windows-x64-Setup
SetupIconFile=..\..\assets\app-icon\filmcraft.ico
UninstallDisplayIcon={app}\{#MyAppExeName}
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
ChangesAssociations=yes

[Languages]
Name: "thai"; MessagesFile: "compiler:Languages\Thai.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"
Name: "associate_fcproj"; Description: "Associate .fcproj files with FilmCraft (เชื่อมโยงไฟล์โปรเจกต์ .fcproj)"; GroupDescription: "File Associations (การเชื่อมโยงไฟล์):"

[Files]
Source: "..\..\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\target\release\{#MyAppCliName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\LICENSE-MIT"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\LICENSE-APACHE"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\ATTRIBUTION.md"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\{#MyAppExeName}"
Name: "{group}\{#MyAppName} CLI"; Filename: "{app}\{#MyAppCliName}"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Registry]
Root: HKA; Subkey: "Software\Classes\.fcproj"; ValueType: string; ValueName: ""; ValueData: "FilmCraft.Project"; Flags: uninsdeletevalue; Tasks: associate_fcproj
Root: HKA; Subkey: "Software\Classes\FilmCraft.Project"; ValueType: string; ValueName: ""; ValueData: "FilmCraft Project"; Flags: uninsdeletekey; Tasks: associate_fcproj
Root: HKA; Subkey: "Software\Classes\FilmCraft.Project\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#MyAppExeName},0"; Tasks: associate_fcproj
Root: HKA; Subkey: "Software\Classes\FilmCraft.Project\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%1"""; Tasks: associate_fcproj

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#MyAppName}}"; Flags: nowait postinstall skipifsilent
