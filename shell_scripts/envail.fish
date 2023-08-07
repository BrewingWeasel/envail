function envail_cd
    set cmd (cargo build -- $argv)
    eval $cmd
end

alias cd envail_cd
