# super_meat_boy_auto_splitter

An experimental port of the [ASL auto splitter for Super Meat Boy](https://github.com/negative-seven/Livesplit.SuperMeatBoy).

Thanks to Thermospore, 6DPSMETA, and ACherryJam for their contributions to the ASL auto splitter.

## Usage

The auto splitter must first be downloaded from the ["Releases" page](https://github.com/negative-seven/super_meat_boy_auto_splitter/releases/) or built from source using `cargo`. The compiled auto splitter is entirely self-contained within the single `.wasm` file.

### [LiveSplit](https://github.com/LiveSplit/LiveSplit)

Follow [these instructions](https://github.com/LiveSplit/LiveSplit.AutoSplitters?tab=readme-ov-file#testing-your-script), choosing the "Auto Splitting Runtime" component instead of the "Scriptable Auto Splitter" component.

### [`obs-livesplit-one`](https://github.com/LiveSplit/obs-livesplit-one)

In the source properties, enable "Use local auto splitter", then provide the path to the auto splitter under "Local Auto Splitter File".

### [`asr-debugger`](https://github.com/LiveSplit/asr-debugger)

Click "Open" next to "WASM File", then select the auto splitter file in the file picker dialog. Note that `asr-debugger` is a tool intended for debugging ASR auto splitters, and not a full-fledged timer.

### Other versions of LiveSplit

Currently, neither the web version nor the desktop version of [LiveSplit One](https://github.com/LiveSplit/LiveSplitOne) support auto splitting.

The [desktop prototype](https://github.com/CryZe/livesplit-one-desktop) of LiveSplit One has had auto splitting support added according to the change history, but it appears not to be functional.
