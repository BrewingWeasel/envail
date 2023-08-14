def _envail_delete_from_active [dirname] {
}


def envail_cd [dir = ""] {
    const nufilerun = '/tmp/envail/nurun'
    envail --shell /usr/bin/nu cd $dir $env.envail_active_dirs | save $nufilerun -f
    print "looooool"
    source $nufilerun
}

$env.envail_active_dirs = []
alias xd = envail_cd
