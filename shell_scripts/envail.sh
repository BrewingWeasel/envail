function envail_cd {
	dir="$(realpath $1)"
	if ! [[ $dir == "$(pwd)*" ]]; then
		if [[ -d .envail/build/bash ]]; then
			source .envail/build/bash/leave
		elif test -f .envail/config.yml; then
			cargo run
			source .envail/build/bash/leave
		fi
	fi
	\cd "$@"
	if [[ -d .envail/build/bash ]]; then
		source .envail/build/bash/enter
	elif test -f .envail/config.yml; then
		cargo run
		source .envail/build/bash/enter
	fi
}
alias cd=envail_cd
