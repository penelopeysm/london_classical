#!/usr/bin/env bash

OWNER=penelopeysm
REPO=london_classical
GIT_DIR="$(git rev-parse --show-toplevel)"
FILENAME="concerts.json"

cd "${GIT_DIR}/src/assets"
gh release upload json ${FILENAME} --clobber

cd ${GIT_DIR}
WORKFLOW=$(ls .github/workflows)
gh workflow enable "${WORKFLOW}"
gh workflow run "${WORKFLOW}" --ref main
