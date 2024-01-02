import {useEffect, useState} from "react";
import init, {start_gameboy} from "gameboy-wasm";


function App() {
  const [ready, setReady] = useState(false)

  useEffect(() => {
    init().then(() => setReady(true))
  }, [])

  const handleButton = () => {
    start_gameboy()
  }

  return (
    <button disabled={!ready} onClick={() => handleButton()}>
      Hello World!
    </button>
  )
}

export default App
