# `dualboot-bluetooth-pairing-keys`

Pure-rust reimagining of [bt-keys-sync](https://github.com/KeyofBlueS/bt-keys-sync), [bluetooth-dualboot](https://github.com/nbanks/bluetooth-dualboot) and similar projects.

The advantage of this is that it doesn't rely on `chntpw`, which nowadays requires a writeable mount of the Windows partition. This tool only needs a read-only mount!
