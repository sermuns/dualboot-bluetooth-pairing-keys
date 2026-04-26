# `dualboot-bt-link-keys`

![demo](media/demo.gif)

Self-contained tool to copy Bluetooth link keys from Windows partitions into your Linux environment.

This problem is explained in https://wiki.archlinux.org/title/Bluetooth#Dual_boot_pairing.

## Why use this over X

The advantage of this tool over others is twofold:

- it doesn't rely on `chntpw`[^1]
- it only needs a read-only mount of the Windows partition (which `chntpw` itself and projects using it listed below, need!)

**Similar projects:**

- [bt-dualboot](https://github.com/x2es/bt-dualboot)
- [bt-keys-sync](https://github.com/KeyofBlueS/bt-keys-sync)
- [bluetooth-dualboot](https://github.com/nbanks/bluetooth-dualboot)

## Installation

From source via

```sh
cargo install dualboot-bt-link-keys
```

or prebuilt binaries  from

1. [GitHub releases](https://github.com/sermuns/dualboot-bt-link-keys/releases/latest)

2. ``sh
   cargo binstall dualboot-bt-link-keys
   ```

## Usage

> [!IMPORTANT]
> We are going to copy the link keys from Windows to Linux!
>
> **First boot into Windows and pair the device(s) you want**, then boot to Linux and follow the instructions below.

1. Find the name of your Windows partition.

   For example:

   ```sh
   lsblk --list --output NAME,FSTYPE,SIZE | grep ntfs
   ```

   (If there are many NTFS partitions, maybe you want the one with the biggest size?)

2. Mount the Windows partition in read-only mode:

   ```sh
   mount --mkdir -o ro /dev/<PARTITION NAME> <MOUNTPOINT>
   ```

3. Run the program with superuser privileges:

   ```sh
   dualboot-bt-link-keys <MOUNTPOINT> --write --restart-bluetooth
   ```

> [!NOTE]
> You can also omit the flags `--write` and `--restart-bluetooth` and run as normal user.
>
> Just manually create/edit the file(s) at `/var/lib/bluetooth/<ADAPTER ADDRESS>/<DEVICE ADDRESS>/info` and add/edit the lines
>
> ```ini
> [LinkKey]
> Key=<LINK KEY>
> ```
>
> Then restart bluetooth with
>
> ```sh
> systemctl restart bluetooth
> ```

4. Unmount the Windows partition:

   ```sh
   umount <MOUNTPOINT>
   ```

5. You are done!

## Disclaimer

This code is 100% certified human-slop. **No artificial intelligence was used in the making of this.**

<a href="https://brainmade.org/">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://brainmade.org/white-logo.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://brainmade.org/black-logo.svg">
  <img alt="brainmade" src="https://brainmade.org/white-logo.svg">
</picture>
</a>

<br>
<br>
<br>

[^1]: https://pogostick.net/~pnh/ntpasswd/
