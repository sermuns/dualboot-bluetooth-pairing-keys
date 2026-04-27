Type "lsblk --list --output NAME,FSTYPE,SIZE | grep ntfs"
Sleep 500ms
Enter
Sleep 2s

Type "sudo mount --mkdir -o ro /dev/nvme0n1p2 /mnt/windows"
Sleep 500ms
Enter
Sleep 4s

Type "dualboot-bt-link-keys /mnt/windows"
Sleep 500ms
Enter
Sleep 2s

Type "sudo dualboot-bt-link-keys /mnt/windows --write --restart-bluetooth"
Sleep 500ms
Enter
Sleep 2s

Type "sudo umount /mnt/windows"
Sleep 500ms
Enter
Sleep 5s
