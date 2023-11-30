#!/usr/bin/env bash

if [ "$1" == "" ]; then
	echo "Must provide the day!"
	exit 1
fi

PROJECT_NAME="day_$1"
DIR="$(cd -P "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cd $DIR/../src

# create the project for this day
cargo new --bin $PROJECT_NAME

# create a directory for the AoC inputs
mkdir $DIR/../src/$PROJECT_NAME/inputs

# automatically add the shared module
echo "shared = { path = \"../shared\" }" >>$DIR/../src/$PROJECT_NAME/Cargo.toml
