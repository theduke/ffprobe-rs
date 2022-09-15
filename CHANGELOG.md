# UNRELEASED

* Add `Default` impl for `ConfigBuilder`
* Add exta field to `FormatTags` [#13](https://github.com/theduke/ffprobe-rs/pull/13) by @imLinguin

# 0.3.3 - 2022-09-05

* Add `Format::get_duration` + `Format::try_get_duration` accessors

# 0.3.2 - 2022-04-23

* FIX: don't fail on missing `Format::size` values.
  Using a default empty string for now.
  The field will be changed to `Option<_>` in the future.

# 0.3.1 - 2022-04-13

* Add configuration system
* Add a `count_frames` setting
  If enabled, the `-count_frames` option will be passed to ffprobe,
  which will do a full decode and count available frames.

# 0.3.0 - 2021-08-02

* Provided more detailed error information
  [#8](https://github.com/theduke/ffprobe-rs/pull/8)
* Make some fields optional

# 0.2.0 - 2021-06-26

* Change `Stream::codec_time_base` to `Option<_>`


