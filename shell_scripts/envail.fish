function envail_cd
    set cmd (envail cd $argv)
    eval $cmd
end

alias cdd envail_cd
