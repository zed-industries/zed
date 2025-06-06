# PowerShell script to rename CodeOrbit to CodeOrbit in all files

$rootDir = Get-Location
$files = Get-ChildItem -Path $rootDir -Recurse -File -Exclude "*.git*", "*.dll", "*.exe", "*.png", "*.jpg", "*.jpeg", "*.gif", "*.ico", "*.svg", "*.woff", "*.woff2", "*.ttf", "*.eot"

# Replace text in files
foreach ($file in $files) {
    try {
        $content = Get-Content $file.FullName -Raw -ErrorAction Stop
        $newContent = $content -replace '\bZed\b', 'CodeOrbit' -replace '\bzed\b', 'codeorbit' -replace '\bZED\b', 'CODEORBIT'
        
        if ($newContent -ne $content) {
            Write-Host "Updating $($file.FullName)"
            Set-Content -Path $file.FullName -Value $newContent -NoNewline -Encoding UTF8
        }
    } catch {
        Write-Warning "Could not process $($file.FullName): $_"
    }
}

# Rename files and directories
Get-ChildItem -Path $rootDir -Recurse -File | 
    Where-Object { $_.Name -match 'CodeOrbit' -or $_.Name -match 'CodeOrbit' } | 
    ForEach-Object {
        $newName = $_.Name -replace 'CodeOrbit', 'codeorbit' -replace 'CodeOrbit', 'CodeOrbit'
        if ($_.Name -ne $newName) {
            $newPath = Join-Path $_.Directory.FullName $newName
            Write-Host "Renaming $($_.FullName) to $newPath"
            Rename-Item -Path $_.FullName -NewName $newPath -ErrorAction SilentlyContinue
        }
    }

Write-Host "Renaming complete!"
