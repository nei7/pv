# pv

`pv` is simple password manager that works in the terminal.

## Security

`pv` uses scrypt for key derivation and aes-256-cbc for encryption. All passwords are kept in a single file.
To access them, you need to provide master password.

## Installation

```bash
git clone https://github.com/nei7/pv
cargo install --path pv
```

On X11 you can use the clipboard to automatically copy passwords in pv to do this you need to install **xclip**

## Usage

### Initializing Vault

```bash
pv init
```

### Adding password

```bash
pv add [name]
```

### Listing all passwords

```bash
pv list
```

### Updating password

```bash
pv update [name]
```

### Deleting passwords

```bash
pv delete [name]
```

### Getting single password

```bash
pv get [name]
```
