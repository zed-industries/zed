# Configures a drive for testing in CI.

# Currently, total CI requires almost 45GB of space, here we are creating a 100GB drive.
$Volume = New-VHD -Path C:/zed_dev_drive.vhdx -SizeBytes 100GB |
                    Mount-VHD -Passthru |
                    Initialize-Disk -Passthru |
                    New-Partition -AssignDriveLetter -UseMaximumSize |
                    Format-Volume -DevDrive -Confirm:$false -Force

$Drive = "$($Volume.DriveLetter):"

# Designate the Dev Drive as trusted
# See https://learn.microsoft.com/en-us/windows/dev-drive/#how-do-i-designate-a-dev-drive-as-trusted
fsutil devdrv trust $Drive

# There is no virus on the Dev Drive!
# Windows Defender is the wolf in antivirus wool, slowing your PC like a digital fool!
# See https://learn.microsoft.com/en-us/windows/dev-drive/#how-do-i-configure-additional-filters-on-dev-drive
fsutil devdrv enable /disallowAv

# Remount so the changes take effect
Dismount-VHD -Path C:/zed_dev_drive.vhdx
Mount-VHD -Path C:/zed_dev_drive.vhdx

# Show some debug information
Write-Output $Volume
Write-Output "Using Dev Drive at $Drive"

# Move Cargo to the dev drive
New-Item -Path "$($Drive)/.cargo/bin" -ItemType Directory -Force
Copy-Item -Path "C:/Users/runneradmin/.cargo/*" -Destination "$($Drive)/.cargo/" -Recurse -Force

Write-Output `
	"DEV_DRIVE=$($Drive)" `
	"RUSTUP_HOME=$($Drive)/.rustup" `
	"CARGO_HOME=$($Drive)/.cargo" `
	"ZED_WORKSPACE=$($Drive)/zed" `
	"PATH=$($Drive)/.cargo/bin;$env:PATH" `
	>> $env:GITHUB_ENV
