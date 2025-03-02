[Setup]
AppId={#AppId}
AppName={#AppName}
AppVerName={#AppDisplayName}
AppPublisher=Zed Industries
AppPublisherURL=https://www.zed.dev/
AppSupportURL=https://www.zed.dev/
AppUpdatesURL=https://www.zed.dev/
DefaultGroupName={#AppName}
AllowNoIcons=yes
OutputDir={#OutputDir}
OutputBaseFilename={#AppSetupName}
Compression=lzma
SolidCompression=yes
AppMutex={#AppMutex}
SetupMutex={#AppMutex}Setup
; WizardImageFile="{#ResourcesDir}\inno-100.bmp,{#ResourcesDir}\inno-125.bmp,{#ResourcesDir}\inno-150.bmp,{#ResourcesDir}\inno-175.bmp,{#ResourcesDir}\inno-200.bmp,{#ResourcesDir}\inno-225.bmp,{#ResourcesDir}\inno-250.bmp"
; WizardSmallImageFile="{#ResourcesDir}\inno-small-100.bmp,{#ResourcesDir}\inno-small-125.bmp,{#ResourcesDir}\inno-small-150.bmp,{#ResourcesDir}\inno-small-175.bmp,{#ResourcesDir}\inno-small-200.bmp,{#ResourcesDir}\inno-small-225.bmp,{#ResourcesDir}\inno-small-250.bmp"
SetupIconFile={#ResourcesDir}\app-icon.ico
UninstallDisplayIcon={app}\{#AppExeName}.exe
ChangesEnvironment=true
ChangesAssociations=true
MinVersion=10.0.16299
SourceDir={#SourceDir}
AppVersion={#Version}
VersionInfoVersion={#Version}
ShowLanguageDialog=auto
WizardStyle=modern

CloseApplications=force

SignTool=Defaultsign
DefaultDirName={autopf}\{#AppName}
PrivilegesRequired=lowest

ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl,{#ResourcesDir}\installer\messages\en.isl"; LicenseFile: "script\terms\terms.rtf"
Name: "simplifiedChinese"; MessagesFile: "{#ResourcesDir}\installer\messages\Default.zh-cn.isl,{#ResourcesDir}\installer\messages\zh-cn.isl"; LicenseFile: "script\terms\terms.rtf"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "addcontextmenufiles"; Description: "{cm:AddContextMenuFiles,{#AppDisplayName}}"; GroupDescription: "{cm:Other}"; Flags: unchecked
Name: "addcontextmenufolders"; Description: "{cm:AddContextMenuFolders,{#AppDisplayName}}"; GroupDescription: "{cm:Other}"; Flags: unchecked; Check: not IsWindows11OrLater
Name: "associatewithfiles"; Description: "{cm:AssociateWithFiles,{#AppDisplayName}}"; GroupDescription: "{cm:Other}"
Name: "addtopath"; Description: "{cm:AddToPath}"; GroupDescription: "{cm:Other}"
Name: "runcode"; Description: "{cm:RunAfter,{#AppDisplayName}}"; GroupDescription: "{cm:Other}"; Check: WizardSilent

[Dirs]
Name: "{app}"; AfterInstall: DisableAppDirInheritance

[Files]
Source: "{#ResourcesDir}\Zed.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#ResourcesDir}\installer\bin\*"; DestDir: "{app}\bin"; Flags: ignoreversion
Source: "{#ResourcesDir}\installer\appx\*"; DestDir: "{app}\appx";  BeforeInstall: RemoveAppxPackage; AfterInstall: AddAppxPackage; Flags: ignoreversion; Check: IsWindows11OrLater

[Icons]
Name: "{group}\{#AppName}"; Filename: "{app}\{#AppExeName}.exe"; AppUserModelID: "{#AppUserId}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExeName}.exe"; Tasks: desktopicon; AppUserModelID: "{#AppUserId}"

[Run]
Filename: "{app}\{#AppExeName}.exe"; Description: "{cm:LaunchProgram,{#AppName}}"; Flags: nowait postinstall; Check: WizardNotSilent
Filename: "{app}\{#AppExeName}.exe"; Description: "{cm:LaunchProgram,{#AppName}}"; Tasks: runcode; Flags: nowait postinstall; Check: WizardSilent

[UninstallRun]
Filename: "powershell.exe"; Parameters: "Invoke-Command -ScriptBlock {{Remove-AppxPackage -Package ""{#AppxFullName}""}"; Check: IsWindows11OrLater; Flags: shellexec waituntilterminated runhidden

[Registry]
Root: HKCU; Subkey: "Software\Classes\.ascx\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.ascx\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.ascx"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ascx"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,ASCX}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ascx"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ascx\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\xml.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ascx\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ascx\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.asp\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.asp\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.asp"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.asp"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,ASP}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.asp"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.asp\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\html.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.asp\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.asp\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.aspx\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.aspx\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.aspx"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.aspx"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,ASPX}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.aspx"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.aspx\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\html.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.aspx\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.aspx\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.bash\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.bash\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.bash"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Bash}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\shell.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.bash_login\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.bash_login\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.bash_login"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash_login"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Bash Login}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash_login"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash_login\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\shell.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash_login\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash_login\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.bash_logout\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.bash_logout\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.bash_logout"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash_logout"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Bash Logout}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash_logout"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash_logout\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\shell.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash_logout\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash_logout\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.bash_profile\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.bash_profile\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.bash_profile"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash_profile"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Bash Profile}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash_profile"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash_profile\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\shell.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash_profile\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bash_profile\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.bashrc\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.bashrc\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.bashrc"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bashrc"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Bash RC}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bashrc"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bashrc\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\shell.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bashrc\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bashrc\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.bib\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.bib\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.bib"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bib"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,BibTeX}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bib"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bib\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bib\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bib\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.bowerrc\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.bowerrc\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.bowerrc"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bowerrc"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Bower RC}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bowerrc"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bowerrc\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\bower.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bowerrc\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.bowerrc\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.c++\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.c++\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.c++"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.c++"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,C++}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.c++"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.c++\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\cpp.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.c++\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.c\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.c\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.c"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.c"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,C}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.c"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.c\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\c.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.c\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.c\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.cc\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.cc\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.cc"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cc"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,C++}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cc"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cc\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\cpp.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cc\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cc\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.cfg\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.cfg\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.cfg"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cfg"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Configuration}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cfg"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cfg\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\config.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cfg\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cfg\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.cjs\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.cjs\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.cjs"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cjs"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,JavaScript}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cjs"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cjs\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\javascript.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cjs\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cjs\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.clj\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.clj\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.clj"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.clj"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Clojure}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.clj"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.clj\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.clj\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.clj\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.cljs\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.cljs\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.cljs"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cljs"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,ClojureScript}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cljs"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cljs\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cljs\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cljs\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.cljx\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.cljx\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.cljx"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cljx"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,CLJX}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cljx"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cljx\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cljx\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cljx\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.clojure\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.clojure\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.clojure"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.clojure"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Clojure}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.clojure"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.clojure\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.clojure\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.clojure\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.cls\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.cls\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.cls"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cls"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,LaTeX}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cls"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cls\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cls\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cls\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.code-workspace\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.code-workspace\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.code-workspace"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.code-workspace"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Code Workspace}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.code"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.code-workspace\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.code-workspace\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.code-workspace\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.cmake\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.cmake\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.cmake"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cmake"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,CMake}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cmake"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cmake\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cmake\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cmake\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.coffee\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.coffee\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.coffee"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.coffee"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,CoffeeScript}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.coffee"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.coffee\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.coffee\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.coffee\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.config\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.config\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.config"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.config"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Configuration}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.config"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.config\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\config.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.config\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.config\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.containerfile\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.containerfile\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.containerfile"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.containerfile"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Containerfile}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.containerfile"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.containerfile\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.containerfile\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.cpp\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.cpp\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.cpp"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cpp"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,C++}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cpp"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cpp\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\cpp.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cpp\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cpp\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.cs\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.cs\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.cs"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cs"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,C#}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cs"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cs\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\csharp.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cs\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cs\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.cshtml\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.cshtml\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.cshtml"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cshtml"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,CSHTML}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cshtml"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cshtml\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\html.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cshtml\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cshtml\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.csproj\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.csproj\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.csproj"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.csproj"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,C# Project}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.csproj"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.csproj\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\xml.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.csproj\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.csproj\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.css\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.css\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.css"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.css"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,CSS}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.css"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.css\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\css.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.css\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.css\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.csv\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.csv\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.csv"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.csv"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Comma Separated Values}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.csv"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.csv\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.csv\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.csv\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.csx\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.csx\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.csx"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.csx"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,C# Script}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.csx"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.csx\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\csharp.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.csx\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.csx\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.ctp\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.ctp\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.ctp"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ctp"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,CakePHP Template}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ctp"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ctp\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ctp\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ctp\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.cxx\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.cxx\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.cxx"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cxx"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,C++}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cxx"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cxx\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\cpp.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cxx\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.cxx\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.dart\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.dart\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.dart"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dart"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Dart}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dart"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dart\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dart\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dart\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.diff\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.diff\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.diff"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.diff"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Diff}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.diff"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.diff\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.diff\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.diff\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.dockerfile\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.dockerfile\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.dockerfile"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dockerfile"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Dockerfile}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dockerfile"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dockerfile\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dockerfile\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dockerfile\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.dot\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.dot\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.dot"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dot"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Dot}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dot"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dot\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dot\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dot\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.dtd\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.dtd\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.dtd"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dtd"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Document Type Definition}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dtd"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dtd\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\xml.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dtd\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.dtd\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.editorconfig\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.editorconfig\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.editorconfig"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.editorconfig"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Editor Config}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.editorconfig"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.editorconfig\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\config.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.editorconfig\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.editorconfig\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.edn\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.edn\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.edn"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.edn"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Extensible Data Notation}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.edn"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.edn\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.edn\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.edn\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.erb\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.erb\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.erb"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.erb"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Ruby}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.erb"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.erb\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\ruby.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.erb\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.erb\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.eyaml\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.eyaml\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.eyaml"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.eyaml"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Hiera Eyaml}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.eyaml"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.eyaml\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\yaml.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.eyaml\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.eyaml\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.eyml\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.eyml\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.eyml"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.eyml"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Hiera Eyaml}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.eyml"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.eyml\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\yaml.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.eyml\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.eyml\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.fs\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.fs\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.fs"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fs"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,F#}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fs"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fs\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fs\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fs\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.fsi\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.fsi\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.fsi"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fsi"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,F# Signature}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fsi"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fsi\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fsi\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fsi\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.fsscript\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.fsscript\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.fsscript"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fsscript"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,F# Script}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fsscript"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fsscript\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fsscript\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fsscript\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.fsx\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.fsx\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.fsx"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fsx"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,F# Script}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fsx"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fsx\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fsx\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.fsx\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.gemspec\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.gemspec\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.gemspec"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gemspec"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Gemspec}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gemspec"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gemspec\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\ruby.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gemspec\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gemspec\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.gitattributes\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.gitattributes\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.gitattributes"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitattributes"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Git Attributes}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitattributes"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitattributes"; ValueType: string; ValueName: "AlwaysShowExt"; ValueData: ""; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitattributes\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\config.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitattributes\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitattributes\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.gitconfig\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.gitconfig\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.gitconfig"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitconfig"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Git Config}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitconfig"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitconfig"; ValueType: string; ValueName: "AlwaysShowExt"; ValueData: ""; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitconfig\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\config.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitconfig\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitconfig\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.gitignore\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.gitignore\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.gitignore"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitignore"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Git Ignore}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitignore"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitignore"; ValueType: string; ValueName: "AlwaysShowExt"; ValueData: ""; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitignore\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\config.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitignore\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gitignore\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.go\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.go\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.go"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.go"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Go}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.go"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.go\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\go.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.go\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.go\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.gradle\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.gradle\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.gradle"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gradle"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Gradle}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gradle"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gradle\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gradle\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.gradle\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.groovy\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.groovy\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.groovy"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.groovy"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Groovy}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.groovy"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.groovy\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.groovy\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.groovy\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.h\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.h\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.h"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.h"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,C Header}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.h"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.h\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\c.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.h\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.h\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.handlebars\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.handlebars\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.handlebars"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.handlebars"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Handlebars}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.handlebars"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.handlebars\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.handlebars\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.handlebars\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.hbs\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.hbs\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.hbs"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hbs"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Handlebars}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hbs"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hbs\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hbs\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hbs\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.h++\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.h++\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.h++"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.h++"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,C++ Header}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.h++"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.h++\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\cpp.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.h++\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.hh\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.hh\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.hh"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hh"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,C++ Header}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hh"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hh\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\cpp.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hh\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hh\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.hpp\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.hpp\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.hpp"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hpp"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,C++ Header}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hpp"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hpp\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\cpp.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hpp\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hpp\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.htm\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.htm\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.htm"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.htm"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,HTML}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.htm"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.htm\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\html.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.htm\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.htm\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.html\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.html\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.html"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.html"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,HTML}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.html"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.html\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\html.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.html\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.html\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.hxx\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.hxx\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.hxx"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hxx"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,C++ Header}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hxx"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hxx\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\cpp.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hxx\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.hxx\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.ini\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.ini\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.ini"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ini"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,INI}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ini"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ini\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\config.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ini\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ini\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.ipynb\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.ipynb\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.ipynb"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ipynb"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Jupyter}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ipynb"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ipynb\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ipynb\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ipynb\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.jade\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.jade\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.jade"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jade"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Jade}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jade"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jade\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\jade.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jade\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jade\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.jav\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.jav\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.jav"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jav"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Java}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jav"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jav\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\java.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jav\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jav\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.java\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.java\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.java"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.java"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Java}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.java"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.java\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\java.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.java\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.java\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.js\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.js\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.js"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.js"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,JavaScript}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.js"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.js\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\javascript.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.js\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.js\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.jsx\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.jsx\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.jsx"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jsx"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,JavaScript}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jsx"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jsx\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\react.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jsx\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jsx\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.jscsrc\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.jscsrc\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.jscsrc"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jscsrc"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,JSCS RC}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jscsrc"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jscsrc\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\javascript.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jscsrc\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jscsrc\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.jshintrc\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.jshintrc\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.jshintrc"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jshintrc"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,JSHint RC}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jshintrc"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jshintrc\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\javascript.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jshintrc\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jshintrc\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.jshtm\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.jshtm\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.jshtm"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jshtm"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,JavaScript HTML Template}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jshtm"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jshtm\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\html.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jshtm\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jshtm\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.json\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.json\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.json"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.json"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,JSON}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.json"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.json\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\json.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.json\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.json\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.jsp\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.jsp\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.jsp"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jsp"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Java Server Pages}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jsp"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jsp\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\html.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jsp\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.jsp\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.less\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.less\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.less"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.less"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,LESS}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.less"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.less\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\less.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.less\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.less\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.log\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.log\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.log"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.log"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Log file}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.log"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.log\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.log\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.log\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.lua\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.lua\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.lua"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.lua"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Lua}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.lua"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.lua\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.lua\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.lua\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.m\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.m\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.m"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.m"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Objective C}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.m"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.m\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.m\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.m\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.makefile\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.makefile\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.makefile"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.makefile"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Makefile}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.makefile"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.makefile\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.makefile\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.makefile\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.markdown\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.markdown\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.markdown"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.markdown"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Markdown}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.markdown"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.markdown\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\markdown.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.markdown\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.markdown\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.md\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.md\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.md"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.md"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Markdown}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.md"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.md\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\markdown.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.md\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.md\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.mdoc\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.mdoc\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.mdoc"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdoc"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,MDoc}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdoc"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdoc\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\markdown.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdoc\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdoc\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.mdown\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.mdown\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.mdown"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdown"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Markdown}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdown"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdown\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\markdown.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdown\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdown\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.mdtext\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.mdtext\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.mdtext"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdtext"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Markdown}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdtext"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdtext\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\markdown.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdtext\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdtext\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.mdtxt\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.mdtxt\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.mdtxt"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdtxt"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Markdown}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdtxt"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdtxt\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\markdown.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdtxt\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdtxt\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.mdwn\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.mdwn\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.mdwn"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdwn"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Markdown}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdwn"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdwn\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\markdown.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdwn\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mdwn\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.mk\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.mk\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.mk"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mk"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Makefile}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mk"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mk\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mk\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mk\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.mkd\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.mkd\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.mkd"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mkd"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Markdown}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mkd"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mkd\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\markdown.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mkd\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mkd\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.mkdn\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.mkdn\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.mkdn"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mkdn"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Markdown}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mkdn"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mkdn\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\markdown.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mkdn\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mkdn\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.ml\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.ml\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.ml"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ml"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,OCaml}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ml"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ml\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ml\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ml\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.mli\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.mli\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.mli"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mli"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,OCaml}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mli"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mli\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mli\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mli\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.mjs\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.mjs\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.mjs"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mjs"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,JavaScript}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mjs"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mjs\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\javascript.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mjs\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.mjs\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.npmignore\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.npmignore\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.npmignore"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.npmignore"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,NPM Ignore}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.npmignore"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.npmignore"; ValueType: string; ValueName: "AlwaysShowExt"; ValueData: ""; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.npmignore\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.npmignore\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.npmignore\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.php\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.php\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.php"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.php"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,PHP}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.php"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.php\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\php.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.php\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.php\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.phtml\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.phtml\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.phtml"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.phtml"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,PHP HTML}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.phtml"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.phtml\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\html.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.phtml\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.phtml\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.pl\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.pl\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.pl"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pl"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Perl}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pl"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pl\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pl\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pl\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.pl6\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.pl6\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.pl6"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pl6"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Perl 6}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pl6"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pl6\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pl6\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pl6\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.plist\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.plist\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.plist"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.plist"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Properties file}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.plist"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.plist\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.plist\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.plist\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.pm\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.pm\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.pm"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pm"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Perl Module}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pm"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pm\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pm\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pm\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.pm6\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.pm6\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.pm6"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pm6"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Perl 6 Module}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pm6"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pm6\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pm6\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pm6\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.pod\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.pod\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.pod"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pod"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Perl POD}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pod"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pod\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pod\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pod\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.pp\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.pp\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.pp"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pp"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Perl}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pp"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pp\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pp\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pp\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.profile\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.profile\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.profile"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.profile"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Profile}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.profile"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.profile\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\shell.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.profile\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.profile\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.properties\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.properties\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.properties"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.properties"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Properties}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.properties"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.properties\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.properties\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.properties\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.ps1\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.ps1\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.ps1"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ps1"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,PowerShell}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ps1"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ps1\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\powershell.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ps1\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ps1\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.psd1\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.psd1\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.psd1"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.psd1"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,PowerShell Module Manifest}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.psd1"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.psd1\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\powershell.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.psd1\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.psd1\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.psgi\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.psgi\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.psgi"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.psgi"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Perl CGI}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.psgi"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.psgi\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.psgi\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.psgi\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.psm1\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.psm1\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.psm1"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.psm1"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,PowerShell Module}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.psm1"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.psm1\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\powershell.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.psm1\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.psm1\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.py\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.py\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.py"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.py"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Python}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.py"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.py\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\python.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.py\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.py\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.pyi\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.pyi\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.pyi"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pyi"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Python}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pyi"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pyi\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\python.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pyi\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.pyi\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.r\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.r\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.r"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.r"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,R}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.r"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.r\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.r\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.r\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.rb\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.rb\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.rb"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rb"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Ruby}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rb"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rb\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\ruby.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rb\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rb\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.rhistory\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.rhistory\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.rhistory"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rhistory"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,R History}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rhistory"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rhistory\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\shell.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rhistory\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rhistory\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.rprofile\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.rprofile\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.rprofile"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rprofile"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,R Profile}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rprofile"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rprofile\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\shell.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rprofile\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rprofile\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.rs\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.rs\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.rs"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rs"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Rust}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rs"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rs\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rs\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rs\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.rst\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.rst\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.rst"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rst"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Restructured Text}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rst"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rst\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rst\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rst\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.rt\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.rt\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.rt"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rt"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Rich Text}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rt"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rt\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rt\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.rt\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.sass\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.sass\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.sass"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.sass"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Sass}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.sass"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.sass\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\sass.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.sass\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.sass\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.scss\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.scss\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.scss"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.scss"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Sass}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.scss"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.scss\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\sass.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.scss\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.scss\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.sh\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.sh\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.sh"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.sh"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,SH}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.sh"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.sh\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\shell.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.sh\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.sh\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.shtml\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.shtml\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.shtml"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.shtml"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,SHTML}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.shtml"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.shtml\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\html.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.shtml\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.shtml\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.sql\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.sql\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.sql"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.sql"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,SQL}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.sql"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.sql\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\sql.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.sql\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.sql\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.svg\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.svg\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.svg"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.svg"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,SVG}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.svg"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.svg\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.svg\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.svg\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.t\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.t\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.t"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.t"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Perl}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.t"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.t\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.t\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.t\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.tex\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.tex\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.tex"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.tex"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,LaTeX}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.tex"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.tex\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.tex\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.tex\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.ts\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.ts\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.ts"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ts"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,TypeScript}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ts"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ts\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\typescript.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ts\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.ts\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.toml\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.toml\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.toml"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.toml"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Toml}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.toml"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.toml\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.toml\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.toml\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.tsx\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.tsx\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.tsx"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.tsx"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,TypeScript}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.tsx"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.tsx\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\react.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.tsx\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.tsx\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.txt\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.txt\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.txt"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.txt"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Text}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.txt"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.txt\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.txt\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.txt\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.vb\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.vb\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.vb"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.vb"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Visual Basic}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.vb"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.vb\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.vb\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.vb\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.vue\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.vue\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.vue"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.vue"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,VUE}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.vue"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.vue\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\vue.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.vue\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.vue\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.wxi\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.wxi\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.wxi"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.wxi"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,WiX Include}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.wxi"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.wxi\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.wxi\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.wxi\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.wxl\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.wxl\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.wxl"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.wxl"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,WiX Localization}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.wxl"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.wxl\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.wxl\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.wxl\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.wxs\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.wxs\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.wxs"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.wxs"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,WiX}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.wxs"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.wxs\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.wxs\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.wxs\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.xaml\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.xaml\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.xaml"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.xaml"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,XAML}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.xaml"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.xaml\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\xml.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.xaml\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.xaml\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.xhtml\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.xhtml\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.xhtml"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.xhtml"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,HTML}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.xhtml"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.xhtml\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\html.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.xhtml\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.xhtml\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.xml\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.xml\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.xml"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.xml"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,XML}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.xml"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.xml\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\xml.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.xml\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.xml\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.yaml\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.yaml\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.yaml"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.yaml"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Yaml}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.yaml"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.yaml\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\yaml.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.yaml\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.yaml\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.yml\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.yml\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.yml"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.yml"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,Yaml}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.yml"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.yml\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\yaml.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.yml\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.yml\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\.zsh\OpenWithProgids"; ValueType: none; ValueName: "{#RegValueName}"; Flags: deletevalue uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\.zsh\OpenWithProgids"; ValueType: string; ValueName: "{#RegValueName}.zsh"; ValueData: ""; Flags: uninsdeletevalue; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.zsh"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,ZSH}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.zsh"; ValueType: string; ValueName: "AppUserModelID"; ValueData: "{#AppUserId}"; Flags: uninsdeletekey; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.zsh\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\shell.ico"; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.zsh\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""; Tasks: associatewithfiles
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}.zsh\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: associatewithfiles

Root: HKCU; Subkey: "Software\Classes\{#RegValueName}SourceFile"; ValueType: string; ValueName: ""; ValueData: "{cm:SourceFile,{#AppName}}"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}SourceFile\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}SourceFile\shell\open"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"""
Root: HKCU; Subkey: "Software\Classes\{#RegValueName}SourceFile\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""

Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}.exe"; ValueType: none; ValueName: ""; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}.exe\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\resources\default.ico"
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}.exe\shell\open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}.exe"""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}.exe\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""

Root: HKCU; Subkey: "Software\Classes\{#RegValueName}ContextMenu"; ValueType: expandsz; ValueName: "Title"; ValueData: "{cm:OpenWithContextMenu,{#ShellNameShort}}"; Tasks: addcontextmenufiles; Flags: uninsdeletekey; Check: IsWindows11OrLater
Root: HKCU; Subkey: "Software\Classes\*\shell\{#RegValueName}"; ValueType: expandsz; ValueName: ""; ValueData: "{cm:OpenWithContextMenu,{#ShellNameShort}}"; Tasks: addcontextmenufiles; Flags: uninsdeletekey; Check: not IsWindows11OrLater
Root: HKCU; Subkey: "Software\Classes\*\shell\{#RegValueName}"; ValueType: expandsz; ValueName: "Icon"; ValueData: "{app}\{#AppExeName}.exe"; Tasks: addcontextmenufiles; Check: not IsWindows11OrLater
Root: HKCU; Subkey: "Software\Classes\*\shell\{#RegValueName}\command"; ValueType: expandsz; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%1"""; Tasks: addcontextmenufiles; Check: not IsWindows11OrLater
Root: HKCU; Subkey: "Software\Classes\directory\shell\{#RegValueName}"; ValueType: expandsz; ValueName: ""; ValueData: "{cm:OpenWithContextMenu,{#ShellNameShort}}"; Tasks: addcontextmenufolders; Flags: uninsdeletekey; Check: IsWindows11OrLater
Root: HKCU; Subkey: "Software\Classes\directory\shell\{#RegValueName}"; ValueType: expandsz; ValueName: "Icon"; ValueData: "{app}\{#AppExeName}.exe"; Tasks: addcontextmenufolders; Check: not IsWindows11OrLater
Root: HKCU; Subkey: "Software\Classes\directory\shell\{#RegValueName}\command"; ValueType: expandsz; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%V"""; Tasks: addcontextmenufolders; Check: not IsWindows11OrLater
Root: HKCU; Subkey: "Software\Classes\directory\background\shell\{#RegValueName}"; ValueType: expandsz; ValueName: ""; ValueData: "{cm:OpenWithContextMenu,{#ShellNameShort}}"; Tasks: addcontextmenufolders; Flags: uninsdeletekey; Check: not IsWindows11OrLater
Root: HKCU; Subkey: "Software\Classes\directory\background\shell\{#RegValueName}"; ValueType: expandsz; ValueName: "Icon"; ValueData: "{app}\{#AppExeName}.exe"; Tasks: addcontextmenufolders; Check: not IsWindows11OrLater
Root: HKCU; Subkey: "Software\Classes\directory\background\shell\{#RegValueName}\command"; ValueType: expandsz; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%V"""; Tasks: addcontextmenufolders; Check: not IsWindows11OrLater
Root: HKCU; Subkey: "Software\Classes\Drive\shell\{#RegValueName}"; ValueType: expandsz; ValueName: ""; ValueData: "{cm:OpenWithContextMenu,{#ShellNameShort}}"; Tasks: addcontextmenufolders; Flags: uninsdeletekey; Check: not IsWindows11OrLater
Root: HKCU; Subkey: "Software\Classes\Drive\shell\{#RegValueName}"; ValueType: expandsz; ValueName: "Icon"; ValueData: "{app}\{#AppExeName}.exe"; Tasks: addcontextmenufolders; Check: not IsWindows11OrLater
Root: HKCU; Subkey: "Software\Classes\Drive\shell\{#RegValueName}\command"; ValueType: expandsz; ValueName: ""; ValueData: """{app}\{#AppExeName}.exe"" ""%V"""; Tasks: addcontextmenufolders; Check: not IsWindows11OrLater

; Environment
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{code:AddToPath|{app}\bin}"; Tasks: addtopath; Check: NeedsAddToPath(ExpandConstant('{app}\bin'))

; URI Scheme
Root: HKCU; Subkey: "Software\Classes\zed"; ValueType: "string"; ValueData: "URL:zed Protocol"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\zed"; ValueType: "string"; ValueName: "URL Protocol"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\zed\DefaultIcon"; ValueType: "string"; ValueData: "{app}\Zed.exe,1"
Root: HKCU; Subkey: "Software\Classes\zed\shell\open\command"; ValueType: "string"; ValueData: """{app}\Zed.exe"" ""%1"""

[Code]
function InitializeSetup(): Boolean;
begin
  Result := True;

  if not WizardSilent() and IsAdmin() then begin
    MsgBox('This User Installer is not meant to be run as an Administrator.', mbError, MB_OK);
    Result := False;
  end;
end;

function WizardNotSilent(): Boolean;
begin
  Result := not WizardSilent();
end;

function IsWindows11OrLater(): Boolean;
begin
  Result := (GetWindowsVersion >= $0A0055F0);
end;

// https://stackoverflow.com/a/23838239/261019
procedure Explode(var Dest: TArrayOfString; Text: String; Separator: String);
var
  i, p: Integer;
begin
  i := 0;
  repeat
    SetArrayLength(Dest, i+1);
    p := Pos(Separator,Text);
    if p > 0 then begin
      Dest[i] := Copy(Text, 1, p-1);
      Text := Copy(Text, p + Length(Separator), Length(Text));
      i := i + 1;
    end else begin
      Dest[i] := Text;
      Text := '';
    end;
  until Length(Text)=0;
end;

function NeedsAddToPath(path: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKCU, 'Environment', 'Path', OrigPath)
  then begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + path + ';', ';' + OrigPath + ';') = 0;
end;

function AddToPath(path: string): string;
var
  OrigPath: string;
begin
  RegQueryStringValue(HKCU, 'Environment', 'Path', OrigPath)

  if (Length(OrigPath) > 0) and (OrigPath[Length(OrigPath)] = ';') then
    Result := OrigPath + path
  else
    Result := OrigPath + ';' + path
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  Path: string;
  InstalledPath: string;
  Parts: TArrayOfString;
  NewPath: string;
  i: Integer;
begin
  if not CurUninstallStep = usUninstall then begin
    exit;
  end;
  if not RegQueryStringValue(HKCU, 'Environment', 'Path', Path)
  then begin
    exit;
  end;
  NewPath := '';
  InstalledPath := ExpandConstant('{app}\bin')
  Explode(Parts, Path, ';');
  for i:=0 to GetArrayLength(Parts)-1 do begin
    if CompareText(Parts[i], InstalledPath) <> 0 then begin
      NewPath := NewPath + Parts[i];

      if i < GetArrayLength(Parts) - 1 then begin
        NewPath := NewPath + ';';
      end;
    end;
  end;
  RegWriteExpandStringValue(HKCU, 'Environment', 'Path', NewPath);
end;

// https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/icacls
// https://docs.microsoft.com/en-US/windows/security/identity-protection/access-control/security-identifiers
procedure DisableAppDirInheritance();
var
  ResultCode: Integer;
  Permissions: string;
begin
  Permissions := '/grant:r "*S-1-5-18:(OI)(CI)F" /grant:r "*S-1-5-32-544:(OI)(CI)F" /grant:r "*S-1-5-11:(OI)(CI)RX" /grant:r "*S-1-5-32-545:(OI)(CI)RX"';

  Permissions := Permissions + Format(' /grant:r "*S-1-3-0:(OI)(CI)F" /grant:r "%s:(OI)(CI)F"', [GetUserNameString()]);

  Exec(ExpandConstant('{sys}\icacls.exe'), ExpandConstant('"{app}" /inheritancelevel:r ') + Permissions, '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
end;

procedure AddAppxPackage();
var
  AddAppxPackageResultCode: Integer;
begin
  if WizardIsTaskSelected('addcontextmenufiles') then begin
    ShellExec('', 'powershell.exe', '-Command ' + AddQuotes('Add-AppxPackage -Path ''' + ExpandConstant('{app}\appx\zed_explorer_command_injector.appx') + ''' -ExternalLocation ''' + ExpandConstant('{app}\appx') + ''''), '', SW_HIDE, ewWaitUntilTerminated, AddAppxPackageResultCode);
    RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\*\shell\{#RegValueName}');
    RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\directory\shell\{#RegValueName}');
    RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\directory\background\shell\{#RegValueName}');
    RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\Drive\shell\{#RegValueName}');
  end;
end;

procedure RemoveAppxPackage();
var
  RemoveAppxPackageResultCode: Integer;
begin
  ShellExec('', 'powershell.exe', '-Command ' + AddQuotes('Remove-AppxPackage -Package ''{#AppxFullName}'''), '', SW_HIDE, ewWaitUntilTerminated, RemoveAppxPackageResultCode);
  if not WizardIsTaskSelected('addcontextmenufiles') then begin
    RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\{#RegValueName}ContextMenu');
  end;
end;

function SwitchHasValue(Name: string; Value: string): Boolean;
begin
  Result := CompareText(ExpandConstant('{param:' + Name + '}'), Value) = 0;
end;

procedure CurStepChanged(CurStep: TSetupStep);
var
  UpdateResultCode: Integer;
	StartServiceResultCode: Integer;
begin
  if CurStep = ssPostInstall then
  begin
    if WizardSilent() then
    begin
      SaveStringToFile(ExpandConstant('{app}\updates\versions.txt'), '{#Version}' + #13#10, True);
    end
  end;
end;
