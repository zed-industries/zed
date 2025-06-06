# Update documentation and comments
$excludeDirs = @('*\.git\*', '*\target\*', '*\node_modules\*', '*\dist\*')

# File types to update (focusing on documentation and source files with comments)
$fileTypes = @('*.md', '*.rs', '*.ts', '*.tsx', '*.js', '*.jsx', '*.css', '*.scss', '*.html', '*.toml', '*.json')

# Simple string replacements
$patterns = @(
    'zedit', 'codeorbit-edit',
    'zeditor', 'codeorbit-editor',
    'Zed', 'CodeOrbit',
    'zed', 'codeorbit',
    'ZED', 'CODEORBIT'
)

# Update file contents
Get-ChildItem -Path . -Recurse -Include $fileTypes -File | 
    Where-Object { 
        $filePath = $_.FullName
        -not ($excludeDirs | Where-Object { $filePath -like $_ })
    } | ForEach-Object {
        $file = $_
        $content = Get-Content -Path $file.FullName -Raw -ErrorAction SilentlyContinue
        if ($null -ne $content) {
            $newContent = $content
            $updated = $false
            
            for ($i = 0; $i -lt $patterns.Count; $i += 2) {
                $pattern = $patterns[$i]
                $replacement = $patterns[$i + 1]
                if ($newContent -match $pattern) {
                    $newContent = $newContent -replace [regex]::Escape($pattern), $replacement
                    $updated = $true
                }
            }
            
            if ($updated) {
                Write-Host "Updating documentation/comments in $($file.FullName)"
                Set-Content -Path $file.FullName -Value $newContent -NoNewline -Encoding UTF8
            }
        }
    }

Write-Host "Documentation and comments update complete!"
