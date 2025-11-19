param(
    [string]$tag = "filezoom-fixtures-baked:latest",
    [string]$testName = "container_isolation_test",
    [string]$testArgs = "-- --nocapture"
)

Write-Output "Running test '$testName' inside baked container $tag (no host mount)..."

# Run container without mounting host filesystem so modifications are isolated
docker run --rm $tag /bin/bash -lc "cd /work/app && BAKED_FIXTURES=1 cargo test -p fileZoom --test $testName $testArgs"
if ($LASTEXITCODE -ne 0) { throw "tests in baked container failed with exit code $LASTEXITCODE" }

Write-Output "Baked-container test completed. Changes were isolated to the container filesystem."
