function _envail_delete_from_active {
	new_array=()
	for value in "${envail_active_dirs[@]}"; do
		[[ $value != $1 ]] && new_array+=($value)
	done
	envail_active_dirs=("${new_array[@]}")
	unset new_array
}

function envail_cd {
	cmd="$(envail --shell /bin/bash cd $1 ${envail_active_dirs[*]})"
	eval $cmd
}

envail_active_dirs=()
alias cd=envail_cd
