# dont-starve-backup

### Overview
The motivation behind creating this tool was to make survival in [Don't Starve](https://www.klei.com/games/dont-starve) less painful.
With the help of a file watcher the tool gets notified whenever the save files are updated. Since the game updates multiple files at a time, the backups are organized by save cycles, then written to a folder with their respective timestamps.

### Usage

Create a file called `config.toml` based on the example found [here](example_config.toml). Update the contents by pointing it at the Don't Starve save directory, the desired output folder:
```toml
save_dir = "/Users/administrator/Library/Application Support/Steam/userdata/123456789/219740/remote"
backup_dir = "/Users/administrator/code/dont-starve-backup/backup"
cycle_interval = 10
```

The tool needs to be run with the following command to start monitoring the save files:

```bash
RUST_LOG=info ./dont-starve-backup monitor

## Sample output
RUST_LOG=info ./target/release/dont-starve-backup monitor
INFO  dont_starve_backup > starting in backup mode
INFO  dont_starve_backup::model > Copied "/Users/administrator/Library/Application Support/Steam/userdata/123456789/219740/remote/boot_modindex" to "/Users/administrator/_Kompi/rust/dont-starve-backup/backup/20220914_100508/boot_modindex"
INFO  dont_starve_backup::model > Copied "/Users/administrator/Library/Application Support/Steam/userdata/123456789/219740/remote/saveindex" to "/Users/administrator/_Kompi/rust/dont-starve-backup/backup/20220914_100508/saveindex"
INFO  dont_starve_backup::model > Copied "/Users/administrator/Library/Application Support/Steam/userdata/123456789/219740/remote/saveindex" to "/Users/administrator/_Kompi/rust/dont-starve-backup/backup/20220914_100508/saveindex"
INFO  dont_starve_backup::model > Copied "/Users/administrator/Library/Application Support/Steam/userdata/123456789/219740/remote/modindex" to "/Users/administrator/_Kompi/rust/dont-starve-backup/backup/20220914_100508/modindex"
INFO  dont_starve_backup::model > Copied "/Users/administrator/Library/Application Support/Steam/userdata/123456789/219740/remote/boot_modindex" to "/Users/administrator/_Kompi/rust/dont-starve-backup/backup/20220914_100508/boot_modindex"
INFO  dont_starve_backup::model > Copied "/Users/administrator/Library/Application Support/Steam/userdata/123456789/219740/remote/motd_image" to "/Users/administrator/_Kompi/rust/dont-starve-backup/backup/20220914_100508/motd_image"
INFO  dont_starve_backup::model > Copied "/Users/administrator/Library/Application Support/Steam/userdata/123456789/219740/remote/motd_image" to "/Users/administrator/_Kompi/rust/dont-starve-backup/backup/20220914_100508/motd_image"
```

To restore a backup, use the following command and pass the directory name that contains the save files:

```bash
RUST_LOG=info ./dont-starve-backup restore 20220912_232323


## Sample output
RUST_LOG=info ./target/release/dont-starve-backup restore backup/20220914_100233
INFO  dont_starve_backup > starting in restore mode
INFO  dont_starve_backup::model > Copied "backup/20220914_100233/motd_image" to "/Users/administrator/Library/Application Support/Steam/userdata/123456789/219740/remote/motd_image"
INFO  dont_starve_backup::model > Copied "backup/20220914_100233/saveindex" to "/Users/administrator/Library/Application Support/Steam/userdata/123456789/219740/remote/saveindex"
INFO  dont_starve_backup::model > Copied "backup/20220914_100233/boot_modindex" to "/Users/administrator/Library/Application Support/Steam/userdata/123456789/219740/remote/boot_modindex"
INFO  dont_starve_backup::model > Copied "backup/20220914_100233/modindex" to "/Users/administrator/Library/Application Support/Steam/userdata/123456789/219740/remote/modindex"
```

## Note on Usage
The tool was only tested on an M1 Mac. No guarantees to work on other OS.
