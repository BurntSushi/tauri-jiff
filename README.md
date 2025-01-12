This repository is meant to provide a means to minimally test [Jiff]'s Android
support. Currently, this just involves printing out some logs and values without
any assertions. It's pretty janky.

To use this, the following steps are required:

* Get Android Studio set up. I did this by installing `android-ndk-25` from the
Archlinux packages and then `android-studio`. Before running `android-studio`,
I set `export ANDROID_NDK_HOME=/opt/android-ndk`.
* Once Android Studio was open, I fumbled around its GUI to create a "device."
I used a Pixel 9 Pro. I followed a bunch of dialogs that I didn't understand.
Then I ran the device.
* Then I ran `cargo tauri android dev` from the root of this repository. The
first time will take a long while.

I'd also like to get Android testing in CI, and it seems like the best bet
there is something like [cargo-dinghy](https://github.com/sonos/dinghy). I'm
very concerned about doing this though because it relies on a lot of other
external tooling to make it work (including spinning up an Android emulator).
I'm already struggling to keep up with CI breaking because of some random
WASM failure.

[Jiff]: https://github.com/BurntSushi/jiff
