# Gatherers - Gather media content from all over the web

- [Overview](#overview)
- [Flow Diagrams](#flow-diagrams)
- [Usage Examples](#usage-examples)

Heavy development

## Overview

Gatherers is a tool to gather media from subscription sites (and likely plenty of other things) and download to your device.
This works off of your own credentials, so you cannot gain access to content you wouldn't normally have access to (Will never support differently).

First available gatherer is for Fansly with OnlyFans under active development.

This is still early times so there is a chance not all content is gathered. This should not be considered a complete archive of your content.

## Flow Diagrams

- [CLI Control Flow](flows/cli.md)
- [Gatherering main logic](flows/gatherering.md)
- [Downloader logic](flows/downloader.md)

## Usage Examples

Currently the CLI is very limited but will be extended heavily in the future.

```shell
# Run gatherers based on the default config
$ gatherers run
Gathering data
fansly: Starting to gather for all subscriptions.
onlyfans: Starting to gather for all subscriptions.
Starting downloader..
fansly: Found 5 subscriptions
onlyfans: Found 13 subscriptions
...
```
