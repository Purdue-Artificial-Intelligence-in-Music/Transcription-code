Goals:
- [ ] Train Mamba on text until the output looks right.
    - Currently loss goes down but output doesn't look correct.
- [ ] Modify the text-based model to take spectogram buckets and output MIDI tokens (like in MT3).
Hopefully by using a RNN to increase the context length to the entire audio file instrument leakage will dissapear.