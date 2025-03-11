import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

import FileUploadSingle from './components/fileUpload'
import Bedgraph, { PlotData } from './components/bedgraph'

function App() {
  const [count, setCount] = useState(0);
  const [bedgraphData, setBedgraphData] = useState<PlotData>(new PlotData(new Map()));

  return (
    <>
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
      <FileUploadSingle setBedgraphData={setBedgraphData}/>
      {bedgraphData.graphs.size > 0 ? (
        <Bedgraph bedgraphData={bedgraphData}/>
      ) : (
        <div>no bedgraph</div>
      )}
    </>
  )
}

export default App
