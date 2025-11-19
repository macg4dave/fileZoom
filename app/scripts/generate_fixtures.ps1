param(
    [int]$count = 300,
    [string]$manifest = "",
    [switch]$append,
    [string]$basePath = ""
)

# Determine fixtures path: prefer explicit param, otherwise use script-relative tests/fixtures.
if ([string]::IsNullOrWhiteSpace($basePath)) {
    $candidate = Join-Path $PSScriptRoot "..\tests\fixtures"
    $resolved = Resolve-Path -Path $candidate -ErrorAction SilentlyContinue
    if ($null -ne $resolved) {
        $basePath = $resolved.Path
    } else {
        # Path doesn't exist yet; use the candidate and ensure it's created below
        $basePath = (Get-Item -Path $candidate -ErrorAction SilentlyContinue | ForEach-Object { $_.FullName })
        if ([string]::IsNullOrWhiteSpace($basePath)) {
            $basePath = $candidate
        }
    }
}

# Ensure the fixtures directory exists
if (-not (Test-Path -Path $basePath)) {
    New-Item -ItemType Directory -Path $basePath -Force | Out-Null
}

# Directory name variants (spaces, unicode, dots, nested levels, weird chars)
$dirs = @(
    "alpha",
    "beta space",
    "unicode-✓",
    "dotfiles",
    ".hidden",
    ".config",
    "mixed.Case",
    "deep\\level1\\level2",
    "weird_chars-!@#",
    "_hidden",
    "numbers_123",
    "multi.ext.tar.gz",
    "camelCaseDir",
    "spaced dir",
    "dir with ünicode",
    "longdirname_" + ("x" * 20)
)

# File suffix / hint variants (unicode, multilingual, symbols)
$suffixes = @("A","B","C","©","数据","こんにちは","Δ","✓","long","x","emoji-😊")

# helper: create random longer filename safely for Windows
function New-SafeName([int]$index,[string]$suffix) {
    $name = "file_{0:000}_{1}.txt" -f $index, $suffix
    if ($name.Length -gt 200) { $name = $name.Substring(0,200) }
    return $name
}

# Create the requested number of files distributed across dirs
for ($i = 1; $i -le $count; $i++) {
    $dirName = $dirs[($i - 1) % $dirs.Count]
    $dirPath = Join-Path $basePath $dirName
    New-Item -ItemType Directory -Path $dirPath -Force | Out-Null

    $suffix = $suffixes[($i - 1) % $suffixes.Count]
    $fileName = New-SafeName -index $i -suffix $suffix
    $filePath = Join-Path $dirPath $fileName

    $content = @()
    $content += ("Fixture file {0} in directory '{1}'" -f $i, $dirName)
    $content += ("Generated: {0}" -f (Get-Date -Format o))
    $content += "Sample text: This file is part of the generated fixtures for fileZoom tests."

    Set-Content -Path $filePath -Value $content -Force -Encoding UTF8
}

# Add some special files with varied extensions and names at the fixtures root
$specialFiles = @(
    "README-copy.md",
    "spaces and tabs.txt",
    "emoji-😊.txt",
    ".gitkeep",
    "archive.sample.tar.gz",
    "binary-blob.bin",
    "dot.trailing",
    "dot.trailing_.txt",
    "COMPLEX.name.with.many.dots.log"
)

foreach ($name in $specialFiles) {
    $p = Join-Path $basePath $name
    # Ensure parent dir exists (root already exists), then create
    New-Item -ItemType File -Path $p -Force | Out-Null
    Set-Content -Path $p -Value (("Special fixture: {0}" -f $name)) -Force -Encoding UTF8
}

# Write a simple manifest (newline-separated relative paths)
if ($manifest -eq "") {
    $manifest = Join-Path $basePath "fixtures_manifest.txt"
}

$allFiles = Get-ChildItem -Path $basePath -Recurse -File -Force | ForEach-Object {
    # produce path relative to fixtures dir using forward slashes
    $rel = $_.FullName.Substring($basePath.Length).TrimStart('\') -replace '\\','/'
    $rel
}

if ($append) {
    $allFiles | Out-File -FilePath $manifest -Encoding UTF8 -Append
} else {
    $allFiles | Out-File -FilePath $manifest -Encoding UTF8 -Force
}

Write-Output "Created $count files in $basePath (plus special files). Manifest: $manifest"