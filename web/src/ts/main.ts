import '../css/main.css';

// globalThis is not implemented by slightly older, ES6-compliant web browsers
import 'core-js/stable/global-this';

import registerFileInputDropEventHandlers from './input_file_drop_zone';
import registerOutputFileSectionEventHandlers from './output_file_section';
import registerFileInputEventHandlers from './input_file_handler';

declare global {
  var outputFileName: string;
}
globalThis.outputFileName = 'optimized_file.ogg';

const optivorbisWorker = new Worker(new URL('./optivorbis_worker.ts', import.meta.url), { name: 'OptiVorbis worker' });

window.addEventListener('DOMContentLoaded', () => {
  registerFileInputEventHandlers(optivorbisWorker);
  registerFileInputDropEventHandlers();
  registerOutputFileSectionEventHandlers();
});
