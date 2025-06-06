# Update file contents
$excludeDirs = @('*\.git\*', '*\target\*')
$filesToUpdate = Get-ChildItem -Path . -Recurse -Include *.rs,*.toml,*.md,*.json -File | 
    Where-Object { 
        $filePath = $_.FullName
        -not ($excludeDirs | Where-Object { $filePath -like $_ })
    }

foreach ($file in $filesToUpdate) {
    $content = Get-Content -Path $file.FullName -Raw -ErrorAction SilentlyContinue
    if ($null -ne $content) {
        $newContent = $content -replace '\bzed_extension_api\b', 'codeorbit_extension_api' `
                              -replace '\bzed_test_extension\b', 'codeorbit_test_extension' `
                              -replace '\bzed_snippets\b', 'codeorbit_snippets' `
                              -replace '\bzed_proto\b', 'codeorbit_proto' `
                              -replace '\bzed_toml\b', 'codeorbit_toml' `
                              -replace '\bzed_ruff\b', 'codeorbit_ruff' `
                              -replace '\bzed_extension\b', 'codeorbit_extension' `
                              -replace '\bzed_industry\b', 'codeorbit_industry' `
                              -replace '\bzed_extension_cli\b', 'codeorbit_extension_cli' `
                              -replace '\bzed_extension_host\b', 'codeorbit_extension_host' `
                              -replace '\bzed_extension_api\b', 'codeorbit_extension_api'
        
        if ($newContent -ne $content) {
            Write-Host "Updating references in $($file.FullName)"
            Set-Content -Path $file.FullName -Value $newContent -NoNewline -Encoding UTF8
        }
    }
}

# Update file and directory names
Get-ChildItem -Path . -Recurse -Directory -Filter "*zed*" | ForEach-Object {
    $newName = $_.Name -replace 'zed', 'codeorbit'
    if ($newName -ne $_.Name) {
        $newPath = Join-Path $_.Parent.FullName $newName
        Write-Host "Renaming directory $($_.FullName) to $newPath"
        Rename-Item -Path $_.FullName -NewName $newPath -ErrorAction SilentlyContinue
    }
}

Get-ChildItem -Path . -Recurse -File -Filter "*zed*" | ForEach-Object {
    $newName = $_.Name -replace 'zed', 'codeorbit'
    if ($newName -ne $_.Name) {
        $newPath = Join-Path $_.DirectoryName $newName
        Write-Host "Renaming file $($_.FullName) to $newPath"
        Rename-Item -Path $_.FullName -NewName $newPath -ErrorAction SilentlyContinue
    }
}

Write-Host "Reference update complete!"
