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
./dont-starve-backup monitor
```

To restore a backup, use the following command and pass the directory name that contains the save files:

```bash
./dont-starve-backup restore 20220912_232323
```