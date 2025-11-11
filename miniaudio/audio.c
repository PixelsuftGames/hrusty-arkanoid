#define MA_NO_WAV
#define MA_NO_FLAC
#define MINIAUDIO_IMPLEMENTATION
#include <stdio.h>
#include "miniaudio.h"

static ma_decoder decoder;
static ma_device_config deviceConfig;
static ma_device device;

void data_callback(ma_device* pDevice, void* pOutput, const void* pInput, ma_uint32 frameCount) {
    ma_decoder* pDecoder = (ma_decoder*)pDevice->pUserData;
    if (pDecoder == NULL)
        return;
    ma_decoder_read_pcm_frames(pDecoder, pOutput, frameCount, NULL);
    (void)pInput;
}

int load_music(const char* path) {
    ma_result result = ma_decoder_init_file(path, NULL, &decoder);
    if (result != MA_SUCCESS)
        return 0;
    ma_data_source_set_looping(&decoder, MA_TRUE);
    return 1;
}

void free_music(void) {
    ma_decoder_uninit(&decoder);
}

int play_music(void) {
    ma_result result = ma_device_start(&device);
    if (result != MA_SUCCESS)
        return 0;
    return 1;
}

int audio_init(void) {
    // TODO: Maybe not hardcode that much?
    deviceConfig = ma_device_config_init(ma_device_type_playback);
    deviceConfig.playback.format   = ma_format_f32;
    deviceConfig.playback.channels = 2;
    deviceConfig.sampleRate        = 44100;
    deviceConfig.dataCallback      = data_callback;
    deviceConfig.pUserData         = &decoder;
    if (ma_device_init(NULL, &deviceConfig, &device) != MA_SUCCESS)
        return 0;
    ma_device_set_master_volume(&device, 0.1f);
    return 1;
}

void audio_destroy(void) {
    ma_device_uninit(&device);
}
