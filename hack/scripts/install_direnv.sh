#!/usr/bin/env bash

DIRENV_VERSION="2.37.1"

function install_direnv() {
	do_install=false
	if command -v direnv; then
		installed_version="$(direnv version)"
		echo "the installed direnv version is $installed_version"
		if [[ "$installed_version" != "$DIRENV_VERSION" ]]; then
			do_install=true
		else
			echo "desired direnv version $DIRENV_VERSION installed"
			exit 0
		fi
	else
		do_install=true
	fi

if $do_install; then
		bin_path="$1" version="v$DIRENV_VERSION" curl -sfL https://direnv.net/install.sh | bash
	fi

}
