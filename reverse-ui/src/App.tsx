import { useState } from 'react'
import reactLogo from './assets/react.svg'
import './App.css'
import {WebTerminal} from "./Terminal";

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="App">
      <WebTerminal api={"/apps/terminal"} params={""} />
    </div>
  )
}

export default App
