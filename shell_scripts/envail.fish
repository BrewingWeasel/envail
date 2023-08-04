function envail_cd
    if test -z $argv[1]
        set dir $HOME
    else
        set dir (path resolve $argv[1])
        # if the dir is a subdirectory, don't undo the configuration
        if string match -q -- "$(pwd)*" "$dir"
            set notnewdir
        end
    end

    if not set -q notnewdir
        if test -d .envail/build/fish
            source .envail/build/fish/leave
        else if test -f .envail/config.yml
            cargo run
            source .envail/build/fish/leave
        end
    end
    builtin cd $argv
    if test -d ".envail/build/fish"
        source .envail/build/fish/enter
    else if test -f ".envail/config.yml"
        cargo run
        source .envail/build/fish/enter
    end
end

alias cd envail_cd
