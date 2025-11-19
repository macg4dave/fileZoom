How to use the fixtures Docker image

1) Build the image (PowerShell):

   ```powershell
   cd app
   .\scripts\build_fixtures_image.ps1 -tag filezoom-fixtures:latest -dockerfile .\docker\Dockerfile -context .
   ```

   Or from WSL / Bash:

   ```bash
   cd app
   ./scripts/build_fixtures_image.sh filezoom-fixtures:latest ./docker/Dockerfile .
   ```

2) Run tests inside the container (PowerShell):

   ```powershell
   cd app
   .\scripts\run_tests_in_docker.ps1 -tag filezoom-fixtures:latest -testArgs "-- --nocapture"
   ```

   Or from WSL / Bash:

   ```bash
   cd app
   ./scripts/run_tests_in_docker.sh filezoom-fixtures:latest 0 "-- --nocapture"
   ```

Notes
- The Docker image copies the repository into the image at `/work` so the container can run tests isolated from the host. Changes inside the container do not persist back to the host unless you mount `-v "$PWD":/work` (the helper scripts mount the repo so you can test the latest local changes; remove the `-v` if you want a purely image-contained run).
- The image includes `attr` tools; xattr commands may be available depending on distribution. ACL and ADS behaviour differs between Windows and Linux — use the respective randomizer script for each OS.
- If you want a purely immutable fixture set per run, build the image and run without the `-v` mount, and tests will run against the fixtures baked into the image.
