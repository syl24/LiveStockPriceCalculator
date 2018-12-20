# cshrc

unset cs_*
set cs_global=/cs/local/generic/lib/global.cshrc
if ( -x /bin/safepath ) set cs_global = `/bin/safepath -q -V4 $cs_global`

if ( "$cs_global" != "" && -r $cs_global) then
  # cs_global is the core of your shell initialization.
  # It will source all the files in ~/csh_init
  # To enable debug mode, simply create the file ~/csh_init/debug   (doesn't matter what's in it.)
  source $cs_global
else
  # No global! Bad.. No other initialization files will be processed.
  # So do some sort of minimal setup
  #
  set path=( /bin /usr/ucb /usr/bin /cs/local/bin . )
  /bin/sh -c 'echo ".cshrc: warning: global cshrc missing" 1>&2'
endif

# Remove path duplicates, keeping first occurrence.
if ($?MANPATH) then
  setenv MANPATH `echo $MANPATH | awk -F: ' BEGIN { OFS=":"; COLON="no" } { for (i=1;i<=NF;i++) if (arr[$i] != 1) { arr[$i]=1; if (COLON == "yes") printf ":"; else COLON = "yes"; printf $i } } '`
endif
if ($?PATH) then
  setenv PATH    `echo $PATH    | awk -F: ' BEGIN { OFS=":"; COLON="no" } { for (i=1;i<=NF;i++) if (arr[$i] != 1) { arr[$i]=1; if (COLON == "yes") printf ":"; else COLON = "yes"; printf $i } } '`
endif

# turbo c shell completions. (TAB) See the man page on tcsh.
if ($?tcsh) then
  complete cd 'p/1/d/'
  complete alias 'p/1/a/'

  set hostnames=( netinfo.ubc.ca interchange.ubc.ca remote.ugrad.cs.ubc.ca prevost bowen pender valdes gambier )
  complete ftp 'p/1/$hostnames/'
  complete telnet 'p/1/$hostnames/'
  complete ssh 'p/*/$hostnames/'
endif
