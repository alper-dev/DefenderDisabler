; Defender Disabler Inno Setup Script
; Requires Inno Setup 7.0+

#define MyAppName "Defender Disabler"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "Defender Disabler"
#define MyAppURL "https://github.com/alper-dev/DefenderDisabler"
#define MyAppExeName "defender-disabler-tray.exe"
#define MyServiceExeName "defender-disabler-service.exe"

[Setup]
AppId={{B1234567-ABCD-1234-ABCD-123456789012}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
OutputDir=installer
OutputBaseFilename=defender-disabler-setup
SetupIconFile=assets\icon.ico
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=admin
ArchitecturesAllowed=x86compatible x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
UninstallDisplayName={#MyAppName}
UninstallDisplayIcon={app}\{#MyAppExeName}

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "startup"; Description: "Start with Windows"; GroupDescription: "Startup:"; Flags: checkedonce

[Files]
; x64 binaries
Source: "target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion; Check: IsX64Compatible
Source: "target\release\{#MyServiceExeName}"; DestDir: "{app}"; Flags: ignoreversion; Check: IsX64Compatible
; x86 binaries
Source: "target\i686-pc-windows-msvc\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion; Check: not IsX64Compatible
Source: "target\i686-pc-windows-msvc\release\{#MyServiceExeName}"; DestDir: "{app}"; Flags: ignoreversion; Check: not IsX64Compatible
; Documentation
Source: "README.md"; DestDir: "{app}"; Flags: ignoreversion isreadme

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\Uninstall {#MyAppName}"; Filename: "{uninstallexe}"

[Registry]
; Auto-start tray app with Windows
Root: HKCU; Subkey: "SOFTWARE\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "DefenderDisabler"; ValueData: """{app}\{#MyAppExeName}"""; Tasks: startup; Flags: uninsdeletevalue

[Run]
; Install and start the service
Filename: "sc.exe"; Parameters: "create DefenderDisabler binPath= ""{app}\{#MyServiceExeName}"" start= auto DisplayName= ""Defender Disabler Service"""; StatusMsg: "Installing service..."; Flags: runhidden waituntilterminated
Filename: "sc.exe"; Parameters: "description DefenderDisabler ""Disables Windows Defender real-time protection to prevent automatic re-enabling."""; StatusMsg: "Setting service description..."; Flags: runhidden waituntilterminated
Filename: "sc.exe"; Parameters: "start DefenderDisabler"; StatusMsg: "Starting service..."; Flags: runhidden waituntilterminated
; Start the tray app
Filename: "{app}\{#MyAppExeName}"; Description: "Launch {#MyAppName}"; Flags: nowait postinstall skipifsilent

[UninstallRun]
; Stop and remove the service
Filename: "sc.exe"; Parameters: "stop DefenderDisabler"; RunOnceId: "StopService"; Flags: runhidden waituntilterminated
Filename: "sc.exe"; Parameters: "delete DefenderDisabler"; RunOnceId: "DeleteService"; Flags: runhidden waituntilterminated
; Kill tray app if running
Filename: "taskkill.exe"; Parameters: "/f /im {#MyAppExeName}"; RunOnceId: "KillTray"; Flags: runhidden waituntilterminated

[UninstallDelete]
Type: filesandordirs; Name: "{app}"

[Code]
// Check if service is already installed
function InitializeSetup(): Boolean;
var
  ResultCode: Integer;
begin
  Result := True;
  
  // Check if service exists
  if Exec('sc.exe', 'query DefenderDisabler', '', SW_HIDE, ewWaitUntilTerminated, ResultCode) then
  begin
    if ResultCode = 0 then
    begin
      // Service exists, stop and remove it first
      Exec('sc.exe', 'stop DefenderDisabler', '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
      Exec('sc.exe', 'delete DefenderDisabler', '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
    end;
  end;
end;

// Kill tray app on uninstall
procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  ResultCode: Integer;
begin
  if CurUninstallStep = usUninstall then
  begin
    Exec('taskkill.exe', '/f /im {#MyAppExeName}', '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
  end;
end;
