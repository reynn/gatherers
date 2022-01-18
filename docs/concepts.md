# Core Concepts

An overview of the foundations of the app, this should help with terminology later or implementing your own versions.

## Gatherers

A `Gatherer` gets media from a source site. Gatherers will generally be implemented as a second crate to keep logic for interacting with individual APIs out of the `core` module.

### Planned Gatherers

- `Fansly`: Can get users paid content, and posts/messages/etc from Fansly
- `OnlyFans`:

### Gatherer Modifiers

A `Modifier` allows gatherers to make changes on behalf of the user back to the Gatherer source.

### Planned Modifiers

- `Liker`: Will apply a like to an object such as post or media bundle on the authed users behalf.

## Downloaders

A `Downloader` takes the content, output from Gatherers, from it's original source and handles archiving it somewhere.

### Downloader Traits

- `BatchDownloader`: Handles incoming downloads and does the setup required for a `FileDownloader`.
  this includes pathing and any threading that may be needed.
- `FileDownloader`: Handles the method of getting the individual file from it's source to it's archived location.

### Planned Batch Downloaders

- `Sequential`: Handles each incoming download and blocks until it completes.
- `MultiThreaded`: Handles downloads by using several workers to download multiple files.
  Worker count is configurable, with a default of 8, a separate thread is used for each.
  Each incoming file is handled exactly once.

### Planned File Downloaders

- `InMemory`: Downloads the file with one request so the body is placed in memory until written to disk.
  This is pretty efficient for smaller files, <10-50Mb, but videos can overwhelm systems with limitted resources.
  Systems with large available memory >16-64Gb can handle this for videos as well.
- `Streaming`: Downloads the file in pieces so that it doesn't overwhelm your system.
