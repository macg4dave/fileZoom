param(
    [string]$tag = "filezoom-fixtures:latest",
    [string]$testArgs = "-- --nocapture",
    [switch]$interactive
)

# Runs cargo test inside the container. Container has the repo copied to /work
if ($interactive) {
    docker run --rm -it -v ${PWD}:/work $tag /bin/bash
    exit $LASTEXITCODE
}

Write-Output "Running tests inside container $tag..."
docker run --rm -v ${PWD}:/work $tag /bin/bash -lc "cd /work/app && cargo test -p fileZoom $testArgs"
if ($LASTEXITCODE -ne 0) { throw "tests in docker failed with exit code $LASTEXITCODE" }

Write-Output "Tests completed inside container."
