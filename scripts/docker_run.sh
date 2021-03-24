#!/usr/bin/env bash

set -e

echo "*** Start SubGame Network ***"

cd $(dirname ${BASH_SOURCE[0]})/..

docker-compose down --remove-orphans
docker-compose up -d

echo "*** SubGame Network is up ***"