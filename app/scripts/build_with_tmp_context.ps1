param(
    [string]$tag = "filezoom-fixtures-baked:tmp",
    [string]$dockerfile = "./docker/Dockerfile",
    [int]$count = 300,
    [switch]$deleteAfter = $true
)

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition

Write-Output "Creating temporary build context..."
$tmp = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), [System.IO.Path]::GetRandomFileName())
New-Item -ItemType Directory -Path $tmp | Out-Null

try {
    Write-Output "Copying repository files into temporary context: $tmp"
    # copy repo into tmp context
    robocopy . $tmp /MIR /XD .git .git\* .github\* /NFL /NDL /NJH /NJS /NP | Out-Null

    # Generate fixtures into tmp context's app/tests/fixtures
    $tmpFixtures = Join-Path $tmp "app\tests\fixtures"
    New-Item -ItemType Directory -Path $tmpFixtures -Force | Out-Null
    Write-Output "Generating fixtures in temporary context: $tmpFixtures"
    & pwsh -NoProfile -ExecutionPolicy Bypass -File (Join-Path $scriptDir "generate_fixtures.ps1") -count $count -manifest (Join-Path $tmpFixtures "fixtures_manifest.txt")

    Write-Output "Building Docker image $tag using temporary context..."
    docker build -t $tag -f $dockerfile $tmp
    if ($LASTEXITCODE -ne 0) { throw "docker build failed with exit code $LASTEXITCODE" }

    Write-Output "Build complete: $tag"

} finally {
    if ($deleteAfter) {
        Write-Output "Removing temporary context $tmp"
        Remove-Item -LiteralPath $tmp -Recurse -Force -ErrorAction SilentlyContinue
    } else {
        Write-Output "Temporary context retained at $tmp"
    }
}
