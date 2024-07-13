pub mod util;
mod consts {
    /// Set to 120 beats per minute.
    pub const BEATS_PER_SECOND: u32 = 120 / 60;
    pub const MICROSECONDS_PER_SECOND: u32 = 1_000_000;
    pub const MICROSECONDS_PER_BEAT: u32 = MICROSECONDS_PER_SECOND / BEATS_PER_SECOND;
    /// Total number of keys/pitches in MIDI.
    pub const MIDI_KEYS: usize = 127;
}
use consts::{MICROSECONDS_PER_BEAT, MIDI_KEYS};

use midi_reader_writer::midly_0_5::merge_tracks;
use midly::Smf;
use tracing::{debug, trace, warn};

/// Series of buckets in discrete time.
/// Each bucket is either on (`true`) or off (`false`)
#[derive(Debug, Default, Clone)]
pub struct PianoRoll {
    pub buckets: Vec<[bool; MIDI_KEYS]>,
}
impl PianoRoll {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
        }
    }
    /// Read all events into [`Self`]
    pub fn from_midi(smf: Smf) -> anyhow::Result<Self> {
        let mut piano_roll = PianoRoll::new();
        piano_roll.buckets.push([false; MIDI_KEYS]);
        for _x @ (tick, _track, event) in merge_tracks(&smf.tracks) {
            // Fill until next event.
            while piano_roll.buckets.len() <= tick.try_into().unwrap() {
                piano_roll
                    .buckets
                    .push(piano_roll.buckets.last().unwrap().clone())
            }
            trace!(?_x);
            match event {
                midly::TrackEventKind::Midi { channel, message } => {
                    if channel.as_int() != 0 {
                        anyhow::bail!("channel: {channel} not 0")
                    }
                    match message {
                        // start note
                        midly::MidiMessage::NoteOn { key, vel } if vel != 0 => {
                            piano_roll.buckets.last_mut().unwrap()[key.as_int() as usize] = true;
                        }
                        // stop note
                        midly::MidiMessage::NoteOff { key, .. }
                        | midly::MidiMessage::NoteOn { key, .. } => {
                            piano_roll.buckets.last_mut().unwrap()[key.as_int() as usize] = false
                        }
                        midly::MidiMessage::ProgramChange { program } => {
                            warn!("TODO: prog: {program}")
                        }
                        midly::MidiMessage::ChannelAftertouch { .. }
                        | midly::MidiMessage::Aftertouch { .. }
                        | midly::MidiMessage::PitchBend { .. }
                        | midly::MidiMessage::Controller { .. } => (),
                    }
                    if !piano_roll.buckets.is_empty() {
                        let dbg_piano = piano_roll
                            .buckets
                            .last()
                            .unwrap()
                            .iter()
                            .copied()
                            .enumerate()
                            .filter(|(_, x)| *x)
                            .collect::<Vec<_>>();
                        debug!(msg=?message, piano_roll = ?dbg_piano);
                    }
                }
                midly::TrackEventKind::Meta(msg) => match msg {
                    midly::MetaMessage::InstrumentName(name) => {
                        debug!(?name);
                    }
                    midly::MetaMessage::Tempo(microseconds) => {
                        if microseconds.as_int() != MICROSECONDS_PER_BEAT {
                            anyhow::bail!("{microseconds} is not {MICROSECONDS_PER_BEAT}")
                        }
                    }
                    // TODO: midly::MetaMessage::TimeSignature(_, _, _, _) => todo!(),
                    // TODO: midly::MetaMessage::KeySignature(_, _) => todo!(),
                    _ => (),
                },
                midly::TrackEventKind::SysEx(_) | midly::TrackEventKind::Escape(_) => (),
            }
        }
        Ok(piano_roll)
    }
}

/* General MIDI specification's program id to instrument:
Prog # Instrument
0.  Acoustic Grand Piano
1.  Bright Acoustic Piano
2.  Electric Grand Piano
3.  Honky-tonk Piano
4.  Electric Piano 1
5.  Electric Piano 2
6.  Harpsichord
7.  Clavi
8.  Celesta
9. Glockenspiel
10. Music Box
11. Vibraphone
12. Marimba
13. Xylophone
14. Tubular Bells
15. Dulcimer
16. Drawbar Organ
17. Percussive Organ
18. Rock Organ
19. Church Organ
20. Reed Organ
21. Accordion
22. Harmonica
23. Tango Accordion
24. Acoustic Guitar (nylon
25. Acoustic Guitar (steel)
26. Electric Guitar (jazz)
27. Electric Guitar (clean)
28. Electric Guitar (muted
29. Overdriven Guitar
30. Distortion Guitar
31. Guitar harmonics
32. Acoustic Bass
33. Electric Bass (finger)
34. Electric Bass (pick)
35. Fretless Bass
36. Slap Bass 1
37. Slap Bass 2
38. Synth Bass 1
39. Synth Bass 2
40. Violin
41. Viola
42. Cello
43. Contrabass
44. Tremolo Strings
45. Pizzicato Strings
46. Orchestral Harp
47. Timpani
48. String Ensemble 1
49. String Ensemble 2
50. SynthStrings 1
51. SynthStrings 2
52. Choir Aahs
53. Voice Oohs
54. Synth Voice
55. Orchestra Hit
56. Trumpet
57. Trombone
58. Tuba
59. Muted Trumpet
60. French Horn
61. Brass Section
62. SynthBrass 1
63. SynthBrass 2
64. Soprano Sax
65. Alto Sax
66. Tenor Sax
67. Baritone Sax
68. Oboe
69. English Horn
70. Bassoon
71. Clarinet
72. Piccolo
73. Flute
74. Recorder
75. Pan Flute
76. Blown Bottle
77. Shakuhachi
78. Whistle
79. Ocarina
80. Lead 1 (square)
81. Lead 2 (sawtooth)
82. Lead 3 (calliope)
83. Lead 4 (chiff)
84. Lead 5 (charang)
85. Lead 6 (voice)
86. Lead 7 (fifths)
87. Lead 8 (bass + lead)
88. Pad 1 (new age)
89. Pad 2 (warm)
90. Pad 3 (polysynth)
91. Pad 4 (choir)
92. Pad 5 (bowed)
93. Pad 6 (metallic)
94. Pad 7 (halo)
95. Pad 8 (sweep)
96.  FX 1 (rain)
97.  FX 2 (soundtrack)
98.  FX 3 (crystal)
99. FX 4 (atmosphere)
100. FX 5 (brightness)
101. FX 6 (goblins)
102. FX 7 (echoes)
103. FX 8 (sci-fi)
104. Sitar
105. Banjo
106. Shamisen
107. Koto
108. Kalimba
109. Bag pipe
110. Fiddle
111. Shanai
112. Tinkle Bell
113. Agogo
114. Steel Drums
115. Woodblock
116. Taiko Drum
117. Melodic Tom
118. Synth Drum
119. Reverse Cymbal
120. Guitar Fret Noise
121. Breath Noise
122. Seashore
123. Bird Tweet
124. Telephone Ring
125. Helicopter
126. Applause
127. Gunshot
*/
