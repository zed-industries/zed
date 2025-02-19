mkdir -p "$env:ZED_WORKSPACE\windows" -ErrorAction Ignore
$channel = $env:RELEASE_CHANNEL
switch ($channel) {
    "stable" {
        $manifestFile = "$env:ZED_WORKSPACE\crates\explorer_command_injector\AppxManifest.xml"
    }
    "preview" {
        $manifestFile = "$env:ZED_WORKSPACE\crates\explorer_command_injector\AppxManifest-Preview.xml"
    }
    default {
        $manifestFile = "$env:ZED_WORKSPACE\crates\explorer_command_injector\AppxManifest-Nightly.xml"
    }
}
Copy-Item -Path "$manifestFile" -Destination "$env:ZED_WORKSPACE\windows\AppxManifest.xml"
# Add makeAppx.exe to Path
$sdk = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64"
$env:Path += ';' + $sdk
makeAppx.exe pack /d "$env:ZED_WORKSPACE\windows" /p "$env:ZED_WORKSPACE\crates\zed\resources\windows\zed_explorer_command_injector.appx" /nv
