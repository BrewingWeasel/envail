if test -d .venv
    source .venv/bin/activate.fish
else if test -d venv
    source venv/bin/activate.fish
else if test -d env
    source env/bin/activate.fish
else if test -d .env
    source .env/bin/activate.fish
end
