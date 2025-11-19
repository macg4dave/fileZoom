param(
    [string]$tag = "filezoom-fixtures:latest",
    [string]$dockerfile = "./docker/Dockerfile",
    [string]$context = ".",
    [int]$count = 300,
    [int]$percentChange = 40,
    [int]$maxAds = 2,
    [switch]$applyAcl,
    [switch]$deleteAfter = $true
)

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$fixturesPath = (Resolve-Path (Join-Path $scriptDir "..\tests\fixtures")).Path

Write-Output "Generating $count fixtures at $fixturesPath..."
& pwsh -NoProfile -ExecutionPolicy Bypass -File (Join-Path $scriptDir "generate_fixtures.ps1") -count $count -manifest (Join-Path $fixturesPath "fixtures_manifest.txt")

Write-Output "Applying random attributes to fixtures (percent=$percentChange maxAds=$maxAds applyAcl=$applyAcl)..."
& pwsh -NoProfile -ExecutionPolicy Bypass -File (Join-Path $scriptDir "randomize_fixture_attrs.ps1") -fixturesPath $fixturesPath -percentChange $percentChange -maxAds $maxAds @(if ($applyAcl) { '-applyAcl' } )

Write-Output "Building Docker image '$tag' using Dockerfile '$dockerfile'..."
docker build -t $tag -f $dockerfile $context
if ($LASTEXITCODE -ne 0) { throw "docker build failed with exit code $LASTEXITCODE" }

if ($deleteAfter) {
    Write-Output "Deleting fixtures folder $fixturesPath (deleteAfter enabled)..."
    Remove-Item -LiteralPath $fixturesPath -Recurse -Force -ErrorAction SilentlyContinue
}

Write-Output "Build complete: $tag"
