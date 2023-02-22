import { useState, useEffect } from 'react';
import {appWindow} from '@tauri-apps/api/window'
import { invoke } from "@tauri-apps/api/tauri"
import { open } from '@tauri-apps/api/dialog';
import { appDir } from '@tauri-apps/api/path';

function App() {
  const [directory, setDirectory] = useState('');

  const handleClick = () => {
    invoke('start_scan')
  }

  const handleStop = () => {
    invoke('stop_scan');
  }

  const handleSelectDirectory = async () => {
    const selected = await open({
      directory: true,
      defaultPath: await appDir(),
    })
    setDirectory(selected);
    invoke('set_directory', { directory: selected });
  }

  const handleStart = async () => {
    invoke('start_scan');
  }

  useEffect(() => {
    appWindow.listen('event-name', (e) => {
      console.log('e:', e)
    })
  }, []);

  return (
    <div className="App">
      <p>{directory}</p>
      <button onClick={handleSelectDirectory}>Open</button>
      <button onClick={handleStart}>start</button>
      {/* <button onClick={handleClick}>start</button>
      <button onClick={handleStop}>stop</button> */}
    </div>
  );
}

export default App;
