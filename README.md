EDSSH
=====

A command-line editor for ssh config files.

### Usage

Assuming the ssh-config file in `~/.ssh/config` looks like this:

```
Host github
    HostName github.com
    IdentityFile ~/.ssh/id_rsa
    Port 22
```

To edit either the `HostName` or the `Port` of the entry `github`, you can run `edssh github`,

```bash
$ edssh github -n github.com.au -p 443
Host github
    HostName github.com.au
    IdentityFile ~/.ssh/id_rsa
    Port 443

```

`edssh` will print the edited version of the ssh config file to stdout.
To apply the change to the ssh config file, add the `-w` option.
If either `-n` or `-p` is not supplied, the existing value will be used.

### Limitations

- only basic ssh-config files are supported, e.g. no comments
- edssh preserves the order of the config file, but may change the format
