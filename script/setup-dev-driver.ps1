# Configures a drive for testing in CI.
# todo(windows)
# The current version of the Windows runner is 10.0.20348 which does not support DevDrive option.
# Ref: https://learn.microsoft.com/en-us/windows/dev-drive/

$Volume = New-VHD -Path C:/zed_dev_drive.vhdx -SizeBytes 30GB |
                    Mount-VHD -Passthru |
                    Initialize-Disk -Passthru |
                    New-Partition -AssignDriveLetter -UseMaximumSize |
                    Format-Volume -FileSystem ReFS -Confirm:$false -Force

$Drive = "$($Volume.DriveLetter):"

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
