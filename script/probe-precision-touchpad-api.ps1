[CmdletBinding()]
param(
    # Parse user32.dll's export directory and report exports that have an
    # ordinal but no name. GetProcAddress cannot find these by name, so a
    # name-only probe says nothing about whether they exist.
    [switch] $ListOrdinalOnlyExports
)

$ErrorActionPreference = "Stop"

# Probes whether the preview Precision Touchpad APIs are reachable, so we can
# decide whether gpui can adopt them behind a runtime presence check.
#
# https://learn.microsoft.com/en-us/windows/win32/input-precisiontouchpad/precision-touchpad-reference
#
# Run on as many Windows builds as possible and report the "Build" line together
# with the table. A function that resolves is not proof that it works, only that
# it is reachable.

if (-not [Environment]::Is64BitProcess) {
    throw "Run this from a 64-bit PowerShell. A 32-bit host resolves against SysWOW64\user32.dll, which is not what Zed loads."
}

Add-Type -Namespace Probe -Name Native -MemberDefinition @"
    [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    public static extern IntPtr LoadLibraryExW(string name, IntPtr file, uint flags);

    [DllImport("kernel32.dll", CharSet = CharSet.Ansi, SetLastError = true, ExactSpelling = true)]
    public static extern IntPtr GetProcAddress(IntPtr module, string name);
"@

$user32Path = Join-Path $env:SystemRoot "System32\user32.dll"

# LOAD_LIBRARY_SEARCH_SYSTEM32, so we never pick up a planted DLL from the CWD.
$module = [Probe.Native]::LoadLibraryExW("user32.dll", [IntPtr]::Zero, 0x800)
if ($module -eq [IntPtr]::Zero) {
    throw "Failed to load user32.dll: $([ComponentModel.Win32Exception]::new([Runtime.InteropServices.Marshal]::GetLastWin32Error()).Message)"
}

# Everything gpui would need to replace the Direct Manipulation viewport with
# real WM_POINTER touchpad gestures. Registration and per-pointer info are
# load-bearing; inertia reporting and injection are quality-of-life.
$functions = @(
    "RegisterTouchpadCapableWindow"
    "RegisterTouchpadCapableThread"
    "GetPointerTouchpadInfo"
    "GetPointerTouchpadInfoHistory"
    "GetPointerFrameTouchpadInfo"
    "GetPointerFrameTouchpadInfoHistory"
    "ReportWindowContentInertia"
    "CreateSyntheticPointerDevice2"
    "InjectTouchpadAction"
    # Baseline: shipped since Windows 8, so it must resolve. If this one comes
    # back missing, the probe itself is broken, not the OS.
    "GetPointerType"
)

$build = Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion"
Write-Host "Build:  $($build.CurrentMajorVersionNumber).$($build.CurrentMinorVersionNumber).$($build.CurrentBuildNumber).$($build.UBR) ($($build.DisplayVersion) $($build.EditionID))"
Write-Host "Module: $((Get-Item $user32Path).VersionInfo.FileVersion)"
Write-Host ""

$results = foreach ($function in $functions) {
    $address = [Probe.Native]::GetProcAddress($module, $function)
    [PSCustomObject]@{
        Function = $function
        Resolved = $address -ne [IntPtr]::Zero
        Address  = if ($address -eq [IntPtr]::Zero) { "" } else { "0x{0:x}" -f $address.ToInt64() }
    }
}

$results | Format-Table -AutoSize

if ($ListOrdinalOnlyExports) {
    $bytes = [IO.File]::ReadAllBytes($user32Path)

    $peHeader = [BitConverter]::ToInt32($bytes, 0x3C)
    if ([BitConverter]::ToUInt32($bytes, $peHeader) -ne 0x00004550) {
        throw "$user32Path is not a PE image."
    }

    $optionalHeader = $peHeader + 24
    $sectionCount = [BitConverter]::ToUInt16($bytes, $peHeader + 6)
    $sectionsStart = $optionalHeader + [BitConverter]::ToUInt16($bytes, $peHeader + 20)
    $isPE32Plus = [BitConverter]::ToUInt16($bytes, $optionalHeader) -eq 0x20B
    $dataDirectories = $optionalHeader + $(if ($isPE32Plus) { 112 } else { 96 })

    $exportRva = [BitConverter]::ToUInt32($bytes, $dataDirectories)
    $exportSize = [BitConverter]::ToUInt32($bytes, $dataDirectories + 4)

    $sections = 0..($sectionCount - 1) | ForEach-Object {
        $header = $sectionsStart + ($_ * 40)
        [PSCustomObject]@{
            VirtualAddress = [BitConverter]::ToUInt32($bytes, $header + 12)
            VirtualSize    = [BitConverter]::ToUInt32($bytes, $header + 8)
            RawDataOffset  = [BitConverter]::ToUInt32($bytes, $header + 20)
        }
    }

    function ConvertTo-FileOffset([uint32] $rva) {
        foreach ($section in $sections) {
            if ($rva -ge $section.VirtualAddress -and $rva -lt ($section.VirtualAddress + $section.VirtualSize)) {
                return $rva - $section.VirtualAddress + $section.RawDataOffset
            }
        }
        throw "RVA 0x{0:x} is not inside any section." -f $rva
    }

    function Read-NullTerminatedString([int] $offset) {
        $end = $offset
        while ($bytes[$end] -ne 0) { $end++ }
        [Text.Encoding]::ASCII.GetString($bytes, $offset, $end - $offset)
    }

    $exportDirectory = ConvertTo-FileOffset $exportRva
    $ordinalBase = [BitConverter]::ToUInt32($bytes, $exportDirectory + 16)
    $functionCount = [BitConverter]::ToUInt32($bytes, $exportDirectory + 20)
    $nameCount = [BitConverter]::ToUInt32($bytes, $exportDirectory + 24)
    $functionTable = ConvertTo-FileOffset ([BitConverter]::ToUInt32($bytes, $exportDirectory + 28))
    $nameOrdinalTable = ConvertTo-FileOffset ([BitConverter]::ToUInt32($bytes, $exportDirectory + 36))

    $namedIndices = [Collections.Generic.HashSet[int]]::new()
    for ($i = 0; $i -lt $nameCount; $i++) {
        [void] $namedIndices.Add([BitConverter]::ToUInt16($bytes, $nameOrdinalTable + ($i * 2)))
    }

    Write-Host "Export directory: $functionCount exports, $nameCount named, ordinal base $ordinalBase"

    $ordinalOnly = for ($index = 0; $index -lt $functionCount; $index++) {
        if ($namedIndices.Contains($index)) { continue }

        $targetRva = [BitConverter]::ToUInt32($bytes, $functionTable + ($index * 4))
        if ($targetRva -eq 0) { continue }

        # An export whose RVA points back inside the export directory is a
        # forwarder, and the string it points at names the real target. That is
        # the only way an unnamed export tells us what it actually is.
        $isForwarder = $targetRva -ge $exportRva -and $targetRva -lt ($exportRva + $exportSize)

        [PSCustomObject]@{
            Ordinal   = $ordinalBase + $index
            Rva       = "0x{0:x}" -f $targetRva
            ForwardsTo = if ($isForwarder) { Read-NullTerminatedString (ConvertTo-FileOffset $targetRva) } else { "" }
        }
    }

    $ordinalOnly = @($ordinalOnly)
    if ($ordinalOnly.Count -eq 0) {
        Write-Host "No ordinal-only exports. Anything absent from the table above is unreachable on this build." -ForegroundColor Yellow
    } else {
        Write-Host "$($ordinalOnly.Count) ordinal-only export(s):"
        $ordinalOnly | Format-Table -AutoSize
    }
}

$missing = @($results | Where-Object { -not $_.Resolved })
if ($missing.Count -eq 0) {
    Write-Host "All probed functions are exported by name from user32.dll." -ForegroundColor Green
    exit 0
}

Write-Host "Not exported by name: $($missing.Function -join ', ')" -ForegroundColor Yellow
exit 1
