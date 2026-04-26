# `dualboot-bt-link-keys`

Pure-Rust, self-contained program to export Bluetooth link keys from Windows partitions when booted in Linux.

## Why use this over X

The advantage of this is that it doesn't rely on `chntpw`, which nowadays requires a writeable mount of the Windows partition. This tool only needs a read-only mount!

**Similar projects:**

- [bt-dualboot](https://github.com/x2es/bt-dualboot)
- [bt-keys-sync](https://github.com/KeyofBlueS/bt-keys-sync)
- [bluetooth-dualboot](https://github.com/nbanks/bluetooth-dualboot)

and this problem is explained in https://wiki.archlinux.org/title/Bluetooth#Dual_boot_pairing.

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
