# joblin

joblin can be run as a background process and execute commands based on individual yaml configuration files.

By default, jobs are searched for in `~/.config/joblin/jobs`.

Example job (executed once each minute):

```yaml
---
job:
  commands:
    - /home/kiwi/bin/check_low_battery.sh
```

This job will be executed once each day at midnight:

```yaml
---
job:
	time:
		minute:
			- 0
		hour:
			- 0
	commands:
		- /home/kiwi/bin/hello_world.sh
		- /home/kiwi/bin/do_something.sh
```

Keys in `time` can be `minute`, `hour`, `day`, `month`, or `weekday`.

## Building
- Clone this repository.
- `cargo build --release` or `cargo install --path .`.
