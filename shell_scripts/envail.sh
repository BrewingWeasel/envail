function envail_cd {
	dir="$(realpath $1)"
	if ! [[ $dir == "$(pwd)*" ]]; then
		if [[ -d .envail/build/ ]]; then
			source .envail/build/leave
		elif test -f .envail/config.yml; then
			cargo run
			source .envail/build/leave
		fi
	fi
	\cd "$@"
	if [[ -d .envail/build/ ]]; then
		source .envail/build/enter
	elif test -f .envail/config.yml; then
		cargo run
		source .envail/build/enter
	fi
}
alias cd=envail_cd
