use macroquad::audio::{load_sound_from_bytes, play_sound, stop_sound, PlaySoundParams, Sound};

static MUSIC_BYTES: &'static [u8] = include_bytes!("music/music.ogg");
static GAME_OVER_SOUND: &'static [u8] = include_bytes!("music/game_over.ogg");

pub struct MusicPlayer {
    enabled: bool,
    main_music: Option<Sound>,
}

impl MusicPlayer {
    pub fn new(enabled: bool) -> Self {
        let main_music = enabled
            .then(|| futures::executor::block_on(load_sound_from_bytes(MUSIC_BYTES)).unwrap());
        if let Some(music) = main_music {
            play_sound(
                music,
                PlaySoundParams {
                    looped: true,
                    volume: 0.5,
                },
            );
        }

        return MusicPlayer {
            enabled,
            main_music,
        };
    }

    pub fn play_game_over(&self) {
        if !self.enabled {
            return;
        }
        if let Some(music) = self.main_music {
            stop_sound(music);
        }
        let sound_effect =
            futures::executor::block_on(load_sound_from_bytes(GAME_OVER_SOUND)).unwrap();
        play_sound(
            sound_effect,
            PlaySoundParams {
                looped: false,
                volume: 1.,
            },
        );
    }
}
