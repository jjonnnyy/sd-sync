import React, {useEffect, useState} from 'react';
import {BsFolder} from 'react-icons/bs';
import {
  selectFolderCallback,
  registerCopyListener,
  registerSkipListener,
  startCopy,
} from './backend';

/* Local storage keys */
const LOCALSTORAGE_SRC = 'src';
const LOCALSTOREAGE_DEST = 'dest';

/**
 * Capitalise the first letter of a string.
 * @param str Input string
 * @returns Modified string
 */
function capitaliseFirstLetter(str: string) {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

/**
 * Truncate folder path to specified length.
 * @param path Path string to truncate
 * @param length Length to truncate to
 * @returns Truncated string
 */
function truncateFolderPath(path: string, length: number): string {
  if (path.length < length) {
    return path;
  }
  const components = path.split('/');
  for (let n = 2; ; n++) {
    const newPath = '.../' + components.slice(n).join('/');
    if (newPath.length < length || n >= components.length - 1) {
      return newPath;
    }
  }
}

const stylePointer: React.CSSProperties = {
  cursor: 'pointer',
};

const stylePadIcon: React.CSSProperties = {
  position: 'relative',
  top: '2px',
  paddingLeft: '5px',
  paddingRight: '5px',
};

/**
 * Main React component for App.
 * @return The JSX to be rendered
 */
export function App(): JSX.Element {
  const sourceDefault = localStorage.getItem(LOCALSTORAGE_SRC);
  const destDefault = localStorage.getItem(LOCALSTOREAGE_DEST);

  const [source, setSource] = useState(sourceDefault);
  const [destination, setDestination] = useState(destDefault);
  const [started, setStarted] = useState(false);
  const [finished, setFinished] = useState(false);
  const [skipped, setSkipped] = useState(0);
  const [copied, setCopied] = useState(0);
  const [error, setError] = useState<string>();

  /* Subscribe to events */
  useEffect(() => {
    const listeners: Promise<() => void>[] = [];

    listeners.push(registerCopyListener(setCopied));
    listeners.push(registerSkipListener(setSkipped));

    return () => {
      listeners.forEach((listener) => {
        listener.then((cb) => cb()).catch(console.error);
      });
    };
  }, []);

  const sourceString = source ? truncateFolderPath(source, 35) :
    'Select source folder';
  const destinationString = destination ? truncateFolderPath(destination, 45) :
    'Select destination folder';

  const srcCallback = selectFolderCallback((path) => {
    localStorage.setItem(LOCALSTORAGE_SRC, path);
    setSource(path);
  });

  const destCallback = selectFolderCallback((path) => {
    localStorage.setItem(LOCALSTOREAGE_DEST, path);
    setDestination(path);
  });

  const disableStart = source == null || destination == null;

  const startClick = () => {
    if (!disableStart) {
      setStarted(true);
      startCopy(source, destination)
          .then(() => setFinished(true))
          .catch((msg: string) => setError(msg));
    }
  };

  /* Start screen */
  if (!started) {
    return (
      <div>
        <h1>SD Sync!</h1>
        <p>
          Copying files from
          <a onClick={srcCallback} style={stylePointer}>
            <BsFolder style={stylePadIcon} />
            {sourceString}
          </a>
        </p>
        <p>
          into
          <a onClick={destCallback} style={stylePointer}>
            <BsFolder style={stylePadIcon} />
            {destinationString}
          </a>
        </p>
        <p>
          <button onClick={startClick} disabled={disableStart}>
            Begin copy
          </button>
        </p>
      </div>
    );
  }

  /* Error screen */
  if (error) {
    return (
      <div>
        <h1>Error</h1>
        <h2>{error}</h2>
      </div>
    );
  }

  /* Copy screen */
  let summaryCount = `Copied ${copied} files`;
  if (skipped > 0) {
    summaryCount += `, skipped ${skipped} files`;
  }

  let summaryHeader = finished ? `Finished ` : '';
  summaryHeader += 'copying files';

  return (
    <div>
      <h1>{capitaliseFirstLetter(summaryHeader)}</h1>
      <h2>{summaryCount}</h2>
    </div>
  );
}
