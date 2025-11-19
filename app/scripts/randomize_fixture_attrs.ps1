param(
    [string]$fixturesPath = (Resolve-Path (Join-Path $PSScriptRoot "..\tests\fixtures")).Path,
    [int]$percentChange = 40,
    [int]$maxAds = 3,
    [switch]$applyAcl
)

Write-Output "Randomizing fixtures in: $fixturesPath"
Write-Output "Chance per file/dir to be modified: $percentChange%"

# Helper: pick if we should modify this entry
function ShouldModify($chancePercent) {
    return (Get-Random -Minimum 0 -Maximum 100) -lt $chancePercent
}

# Helper: pick a subset of attribute flags to set
function PickAttributes() {
    $flags = @('ReadOnly','Hidden','System','Archive')
    $picked = @()
    foreach ($f in $flags) {
        if (ShouldModify(30)) { $picked += $f }
    }
    if ($picked.Count -eq 0 -and (ShouldModify(10))) { $picked += 'ReadOnly' }
    return ($picked -join ',')
}

# Helper: randomly set times
function RandomizeTimes($item) {
    $now = Get-Date
    $daysBack = Get-Random -Minimum 0 -Maximum 365
    $randDate = $now.AddDays(-$daysBack).AddSeconds(- (Get-Random -Minimum 0 -Maximum 86400))
    try {
        $item.CreationTime = $randDate
        $item.LastWriteTime = $randDate.AddSeconds((Get-Random -Minimum 0 -Maximum 10000))
        $item.LastAccessTime = $randDate.AddSeconds((Get-Random -Minimum 0 -Maximum 10000))
    } catch {
        # ignore if not supported
    }
}

# Helper: add small ADS streams
function AddRandomADS($filePath, $maxAds) {
    $adsCount = Get-Random -Minimum 0 -Maximum ($maxAds + 1)
    for ($a = 0; $a -lt $adsCount; $a++) {
        $name = "meta$a"
        try {
            $adsPath = $filePath + ':' + $name
            Set-Content -Path $adsPath -Value (("ADS {0} created on {1}" -f $name, (Get-Date -Format o))) -ErrorAction SilentlyContinue
        } catch {
            # ignore
        }
    }
}

# Helper: apply ACL change (grant or deny) using icacls for safety. Only do a few.
function ApplyRandomAcl($path) {
    # choose principal - use 'Everyone' for simplicity
    $principal = 'Everyone'
    $permOptions = @('R','W','F')
    $perm = $permOptions[(Get-Random -Minimum 0 -Maximum $permOptions.Count)]
    $action = (Get-Random -Minimum 0 -Maximum 100)
    try {
        if ($action -lt 65) {
            # grant
            icacls "$path" /grant "${principal}:($perm)" | Out-Null
        } else {
            # deny (be cautious)
            icacls "$path" /deny "${principal}:($perm)" | Out-Null
        }
    } catch {
        # ignore ACL failures
    }
}

# Collect files and directories
$entries = Get-ChildItem -Path $fixturesPath -Recurse -Force -ErrorAction SilentlyContinue

$modified = 0
foreach ($e in $entries) {
    # Decide randomly whether to modify
    if (-not (ShouldModify $percentChange)) { continue }

    $modified++
    # Randomly set attributes
    $attrString = PickAttributes
    if ($attrString -ne '') {
        try {
            # Convert flag string to attribute enum value safely
            $attrs = [System.IO.FileAttributes]::Normal
            foreach ($part in $attrString -split ',') {
                $p = $part.Trim()
                if ($p -ne '') {
                    $attrs = $attrs -bor ([System.IO.FileAttributes]::$p)
                }
            }
            # some items are directories
            try {
                $e.Attributes = $attrs
            } catch {
                # fallback: use attrib.exe
                attrib +$attrString "$($e.FullName)" 2>$null
            }
        } catch {
            # ignore attribute failures
        }
    }

    # Randomly randomize times
    if (ShouldModify 60) { RandomizeTimes $e }

    # Add ADS for files
    if ($e.PSIsContainer -ne $true) {
        if (ShouldModify 30) { AddRandomADS $e.FullName $maxAds }
    }

    # Optionally apply ACLs (slower) - limit amount
    if ($applyAcl) {
        if (ShouldModify 15) { ApplyRandomAcl $e.FullName }
    }
}

Write-Output "Randomization complete. Modified approx $modified entries."
Write-Output "Note: Some operations (ACLs, ADS, attributes) may be skipped on files where the OS or permissions prevent changes."