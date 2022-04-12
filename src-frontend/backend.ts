import {invoke} from '@tauri-apps/api';
import {open} from '@tauri-apps/api/dialog';
import {appWindow} from '@tauri-apps/api/window';

/* Tauri events emitted by backend */
const EVENT_COPY = 'copy';
const EVENT_SKIP = 'skip';

type countCallback = (count: number) => void;

/**
 * Register callback for the file copy event.
 * @param cb Callback to be registered
 * @returns Promise resolving to unlisten callback
 */
export function registerCopyListener(cb: countCallback) : Promise<() => void> {
  return appWindow.listen(EVENT_COPY, (event) => {
    if (isCountPayload(event.payload)) {
      cb(event.payload.count);
    } else {
      throw Error('Unexpected event payload');
    }
  });
}

/**
 * Register callback for the file skip event.
 * @param cb Callback to be registered
 * @returns Promise resolving to unlisten callback
 */
export function registerSkipListener(cb: countCallback) : Promise<() => void> {
  return appWindow.listen(EVENT_SKIP, (event) => {
    if (isCountPayload(event.payload)) {
      cb(event.payload.count);
    } else {
      throw Error('Unexpected event payload');
    }
  });
}

/* Payload type returned by copy and skip events */
interface CountPayload {
  count: number;
}

/**
 * CountPayload typeguard.
 * @param payload Unknown payload object
 * @returns True if payload is CountPayload
 */
function isCountPayload(payload: unknown): payload is CountPayload {
  return payload != null && typeof payload == 'object' && 'count' in payload;
}

/* Tauri commands available on backend */
const COMMAND_START = 'start_copy';

/**
 * Send start copy command to backend.
 * @param source Folder path to copy from
 * @param destination Folder path to copy into
 * @returns Promise that resolves/errors on copmletion
 */
export function startCopy(source: string, destination: string)
  : Promise<unknown> {
  return invoke(COMMAND_START, {source, destination});
}

/**
 * Generate callback function for selecting a folder.
 * @param cb Callback to executed on result of folder selection
 * @return Generated callback function
 */
export function selectFolderCallback(cb: (str: string) => void) {
  return () => {
    open({directory: true, multiple: false})
        .then((path) => {
          if (path != null) {
          /* cannot be array as we've set multiple:false */
            cb(path as string);
          }
        })
        .catch(console.error);
  };
}
