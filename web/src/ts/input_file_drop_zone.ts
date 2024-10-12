const dropHighlightElementClasses = ['bg-sky-300', 'dark:bg-sky-600'];
const dropEffect = 'copy';

const fileInput = document.getElementById('file-input') as HTMLInputElement;
const fileInputButton = document.getElementById('file-input-button') as HTMLButtonElement;
const fileInputLabel = document.getElementById('file-input-label') as HTMLLabelElement;

function getDraggedAudioFile(dataTransfer: DataTransfer | null): File | boolean {
  let audioFile: File | boolean = false;

  if (dataTransfer) {
    for (let i = 0; i < dataTransfer.items.length && !audioFile; i += 1) {
      const item = dataTransfer.items[i];
      const itemFile = item.getAsFile();

      // Browsers may categorize Ogg Vorbis files inconsistently, using
      // the audio, application and video types. Accept anything that
      // may be an Ogg Vorbis file to compensate
      if (item.type.match(/^(?:audio|application|video)\/ogg/)) {
        audioFile = itemFile ?? true;
      }
    }
  }

  return audioFile;
}

function handleAudioFileDragBegin(event: DragEvent) {
  const { dataTransfer } = event;
  const vorbisFile = getDraggedAudioFile(dataTransfer);

  if (vorbisFile) {
    dataTransfer!.dropEffect = dropEffect;

    fileInputLabel.classList.add(...dropHighlightElementClasses);

    event.preventDefault();
  }
}

function handleAudioFileDragStop() {
  fileInputLabel.classList.remove(...dropHighlightElementClasses);
}

function handleAudioFileDrop(event: DragEvent) {
  const { dataTransfer } = event;
  const vorbisFile = getDraggedAudioFile(dataTransfer);

  if (vorbisFile instanceof File) {
    const dummyDataTransfer = new DataTransfer();
    dummyDataTransfer.items.add(vorbisFile);
    fileInput.files = dummyDataTransfer.files;
    fileInput.dispatchEvent(new Event('input'));

    fileInputLabel.classList.remove(...dropHighlightElementClasses);

    event.preventDefault();
  }
}

export default function registerFileInputDropEventHandlers() {
  for (const element of [fileInputLabel, fileInputButton]) {
    (element as HTMLElement).addEventListener('dragenter', handleAudioFileDragBegin);
    (element as HTMLElement).addEventListener('dragover', handleAudioFileDragBegin);
    (element as HTMLElement).addEventListener('dragleave', handleAudioFileDragStop);
    (element as HTMLElement).addEventListener('drop', handleAudioFileDrop);
  }
  fileInputButton.addEventListener('click', () => fileInput.click());
}
