## Rustmarks

- lightweight alternative to zshmarks

- zshmarks needs oh-my-zsh which makes the shell prompt slightly slower


## Use

- add `rustmarks` into your $PATH

- `rustmarks add <alias>` will add $CWD into the bookmarks

- `rustmarks jump <alias>` print the associated directory to stdout (cant change shell directory from within); use that output with the following alias:


```
# rustmarks jump alias
function j {
  target=$(rustmarks jump $1)
  retval=$?
  if [ $retval -eq 0 ]; then
     cd $target
  fi
}
```