# `dualboot-bt-link-keys`

![demo](media/demo.gif)

Self-contained tool to copy Bluetooth link keys from Windows partitions into your Linux environment.

This problem is explained in https://wiki.archlinux.org/title/Bluetooth#Dual_boot_pairing.

## Why use this over X

The advantage of this tool over others is twofold:

- it doesn't rely on `chntpw`
- it only needs a read-only mount of the Windows partition (which `chntpw` doesn't support anymore)

**Similar projects:**

- [bt-dualboot](https://github.com/x2es/bt-dualboot)
- [bt-keys-sync](https://github.com/KeyofBlueS/bt-keys-sync)
- [bluetooth-dualboot](https://github.com/nbanks/bluetooth-dualboot)

## Installation

Not published yet. For now:

```sh
cargo install --git https://github.com/sermuns/dualboot-bt-link-keys
```

## Usage

1. Find your Windows partition

   > [!TIP]
   > Search for a partition with NTFS filesystem by:
   >
   > ```sh
   > lsblk --list --output NAME,FSTYPE | grep ntfs
   > ```

2. Mount the Windows partition in read-only mode

   ```sh
   mount --mkdir -o ro <YOUR PARTITION HERE> <DESIRED MOUNTPOINT>
   ```

3. Run the program

   ```sh
   dualboot-bt-link-keys <DESIRED MOUNTPOINT> --write --restart-bluetooth
   ```

4. It should have written link keys to correct location, try connecting!


## Disclaimer

This code is 100% certified human-slop. **No artificial intelligence was used in the making of this.**

<a href="https://brainmade.org/">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://brainmade.org/white-logo.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://brainmade.org/black-logo.svg">
  <img alt="brainmade" src="https://brainmade.org/white-logo.svg">
</picture>
</a>
