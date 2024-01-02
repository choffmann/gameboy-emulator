import {useEffect, useState} from "react";
import init, {load_boot_rom, CPU} from "gameboy-wasm";


function App() {
  const [ready, setReady] = useState(false)

  useEffect(() => {
    init().then(() => setReady(true))
  }, [])

  const handleButton = () => {
    const cpu = new CPU();
    const cpuAsJson = cpu.to_json();
    const memory = cpuAsJson.memory;
    console.log("Memory", memory)
  }

  return (
      <button disabled={!ready} onClick={() => handleButton()}>
        Run
      </button>
  )
}

export default App
