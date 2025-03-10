import { OggToOgg } from 'optivorbis';

self.addEventListener('message', (ev: MessageEvent<File>) => {
  try {
    const vorbisFileData = new Uint8Array(new FileReaderSync().readAsArrayBuffer(ev.data));
    const optimizedVorbisFileData = new OggToOgg().remux(vorbisFileData);
    self.postMessage(new Blob([optimizedVorbisFileData], { type: 'audio/ogg' }));
  } catch (remuxErrorMessage) {
    // Error handling in web workers is finicky, so we pass an error message to the main thread
    self.postMessage((remuxErrorMessage as object).toString());
  }
});
