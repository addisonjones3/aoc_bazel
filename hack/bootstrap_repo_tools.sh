#!/usr/bin/env bash

source ./hack/scripts/install_direnv.sh

bin_path="$HOME/.local/bin"

function install_all() {
	install_direnv bin_path
}

install_all
