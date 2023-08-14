for venvname in .venv venv env .env
    if test -d $venvname
        source $venvname/bin/activate.fish
    end
end
