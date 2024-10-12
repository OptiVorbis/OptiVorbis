const inputFileSection = document.getElementById('file-input-section')!;
const outputFileSection = document.getElementById('output-file-section')!;
const outputFilePreviewPlayer = document.getElementById('output-file-preview') as HTMLAudioElement;
const outputFileSectionBackButton = document.getElementById('output-file-back') as HTMLButtonElement;
const outputFileSectionDownloadButton = document.getElementById('output-file-download') as HTMLButtonElement;

export default function registerOutputFileSectionEventHandlers() {
  outputFileSectionBackButton.addEventListener('click', () => {
    const filePreviewURL = outputFilePreviewPlayer.src;
    if (filePreviewURL) {
      outputFilePreviewPlayer.pause();
      URL.revokeObjectURL(filePreviewURL);
    }

    inputFileSection.classList.remove('hidden');
    outputFileSection.classList.add('hidden');
  });
  outputFileSectionDownloadButton.addEventListener('click', () => {
    const anchor = document.createElement('a');
    anchor.href = outputFilePreviewPlayer.src;
    anchor.download = globalThis.outputFileName;
    anchor.click();
  });
}
