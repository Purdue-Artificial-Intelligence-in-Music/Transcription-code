# Transcription

Converting audio files into symbols (midi/sheet music).

# Build tools

- The python code requires installing the [python interpreter](https://www.python.org/downloads/) and either [`conda`](https://docs.anaconda.com/miniconda/) or [`mamba`](https://mamba.readthedocs.io/en/latest/installation/micromamba-installation.html)(faster). Then `conda env create --file deps.yaml`
- The rust code requires installing [the rust compiler/toolchain](`https://www.rust-lang.org/tools/install`). No libraries must be manually handled.

# Running

Enter the directory of the code to run and then:
- The python code is run with `python <file.py>`.
- The rust code is run with `cargo run [--release]`