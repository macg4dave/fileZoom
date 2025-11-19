# Image for running fileZoom tests with fixtures baked in
# Builds a container that contains the repository at /work and fixtures at /work/app/tests/fixtures
FROM rust:1.72-bullseye

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
    && apt-get install -y --no-install-recommends git ca-certificates attr file tzdata \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /work

# Copy repo into image. This preserves the fixtures under app/tests/fixtures
COPY . /work

# If an in-image permission script exists, run it now (apply randomized perms inside image)
RUN if [ -x /work/app/scripts/apply_permissions_in_image.sh ]; then /bin/bash /work/app/scripts/apply_permissions_in_image.sh; fi

WORKDIR /work/app

# Make scripts executable
RUN chmod +x scripts/*.sh || true

# Default command: open a shell. Use the helper run script to execute tests.
CMD ["/bin/bash"]
