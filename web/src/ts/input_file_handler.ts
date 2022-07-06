import showFileErrorModal from './file_error_modal';

const optivorbisLogo = document.getElementById('optivorbis-logo')!;
const inputFileSection = document.getElementById('file-input-section')!;
const fileInput = <HTMLInputElement>document.getElementById('file-input');
const fileProcessText = document.getElementById('file-process-text')!;
const outputFilePreviewPlayer = <HTMLAudioElement>document.getElementById('output-file-preview');
const outputFileSection = document.getElementById('output-file-section')!;

function showProgressElements() {
  optivorbisLogo.classList.add('animate-pulse');
  inputFileSection.classList.add('hidden');
  fileProcessText.classList.remove('hidden');
}

function hideProgressElements() {
  optivorbisLogo.classList.remove('animate-pulse');
  inputFileSection.classList.remove('hidden');
  fileProcessText.classList.add('hidden');
}

export default function registerFileInputEventHandlers(optivorbisWorker: Worker) {
  fileInput.addEventListener('input', () => {
    if (!fileInput.files || fileInput.files.length === 0) {
      return;
    }

    const vorbisFile = fileInput.files[0];
    globalThis.outputFileName = vorbisFile.name;

    showProgressElements();

    optivorbisWorker.postMessage(vorbisFile);

    optivorbisWorker.onerror = hideProgressElements;
    optivorbisWorker.onmessageerror = hideProgressElements;
    optivorbisWorker.onmessage = (ev: MessageEvent<Blob | string>) => {
      hideProgressElements();

      if (typeof ev.data !== 'string') {
        const filePreviewURL = outputFilePreviewPlayer.src;
        if (filePreviewURL) {
          outputFilePreviewPlayer.pause();
          URL.revokeObjectURL(filePreviewURL);
        }
        outputFilePreviewPlayer.src = URL.createObjectURL(ev.data);

        inputFileSection.classList.add('hidden');
        outputFileSection.classList.remove('hidden');
      } else {
        showFileErrorModal(ev.data);
      }
    };

    // Some browsers only fire this event if a different file is
    // selected, so make sure that selecting the same file again
    // triggers this event again no matter what by clearing the
    // selection
    fileInput.files = new DataTransfer().files;
  });
}
