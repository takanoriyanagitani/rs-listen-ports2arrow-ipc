#!/bin/sh

./listen-ports2arrow-ipc |
	rs-ipc-stream2df \
		--sql '
			SELECT * FROM ipc_table
			ORDER BY process_name
		' |
	arrow-ipc-stream2jsonl |
	jq -c |
	json-filter-cel \
		--json-obj-name=item \
		'
			(item.protocol == "tcp")
			&& (
				!(
					item.process_name in [
						"Python",
						"ssh",
						"sshd-session",
						"pkgsite",
						"miniserve",
						"Notes",
						"sccache",
						"ollama",
						"firefox",
						"ControlCenter",
					]
				)
			)
			&& (! item.process_name.contains("mqtt"))
			&& (! item.process_path.contains("Private"))
			&& (! item.process_name.contains("Adobe"))
			&& (! item.process_path.contains("github"))
			&& (! item.process_name.startsWith("Parallels"))
		'
