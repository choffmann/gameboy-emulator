import {useEffect, useState} from "react";
import init, {greet} from '../../gameboy-wasm/pkg'
function App() {
  const [ready, setReady] = useState(false)

  useEffect(() => {
    init().then(() => setReady(true))
  }, [])

  const handleButton = () => {
    greet()
  }

  return (
    <button disabled={!ready} onClick={() => handleButton()}>
      Hello World!
    </button>
  )
}

export default App
