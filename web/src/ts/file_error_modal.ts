const fileErrorModal = document.getElementById('file-error-modal')!;
const fileErrorModalMessage = document.getElementById('file-error-modal-message')!;

document.getElementById('file-error-modal-close')!.addEventListener('click', () => {
  fileErrorModal.classList.add('hidden');
});

export default function showFileErrorModal(errorMessage: string) {
  fileErrorModalMessage.innerText = errorMessage;
  fileErrorModal.classList.remove('hidden');
}
