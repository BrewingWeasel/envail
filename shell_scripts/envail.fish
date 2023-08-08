function _envail_delete_from_active
    set -l index (contains -i -- $argv[1] $envail_active_dirs)
    if set -q index[1]
        set -e envail_active_dirs[$index]
    else
        return 1
    end
end


function envail_cd
    set cmd (envail cd $argv $envail_active_dirs)
    eval $cmd
end

set envail_active_dirs
alias cd envail_cd
