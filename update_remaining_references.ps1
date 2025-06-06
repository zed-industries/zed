# Update file contents
$excludeDirs = @('*\.git\*', '*\target\*', '*\node_modules\*', '*\dist\*')

# Update file contents
Get-ChildItem -Path . -Recurse -Include *.rs,*.toml,*.md,*.json,*.html,*.ts,*.tsx,*.js,*.jsx,*.css,*.scss -File | 
    Where-Object { 
        $filePath = $_.FullName
        -not ($excludeDirs | Where-Object { $filePath -like $_ })
    } | ForEach-Object {
        $file = $_
        $content = Get-Content -Path $file.FullName -Raw -ErrorAction SilentlyContinue
        if ($null -ne $content) {
            $newContent = $content -replace '\bzed_', 'codeorbit_'
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
