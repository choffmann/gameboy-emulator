import {useEffect, useState} from "react";
import init, {CPU} from "gameboy-wasm";
import {Box, CircularProgress, Container} from "@mui/material";
import ControlCenter from "./components/ControlCenter.tsx";


function App() {
  const [ready, setReady] = useState(false)

  useEffect(() => {
    init().then(() => setReady(true))
  }, [])

  const handlePlay = () => {
    const cpu = new CPU();
    const cpuAsJson = cpu.to_json();
    const memory = cpuAsJson.memory;
    const register = cpuAsJson.register;
    console.log("CPU", cpuAsJson)
    console.log("Registers", register)
    console.log("Memory", memory)
  }

  const LoadingCircle = () => {
    return (
        <Box sx={{
          position: "absolute",
          top: "50%",
          left: "50%",
          transform: "translate(-50%, -50%)"
        }}>
          <CircularProgress/>
        </Box>
    )
  }

  const OnReady = () => {
    return (
        <>
          <ControlCenter onPlay={handlePlay}/>
        </>
    )
  }

  return (
      <Container>
        {ready ? <OnReady/> : <LoadingCircle/>}
      </Container>
  )
}

export default App
