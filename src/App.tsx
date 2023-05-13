import ReactFlow, {
  MiniMap,
  Controls,
  Background,
} from 'reactflow';

import 'reactflow/dist/style.css';

import { Button } from 'semantic-ui-react';

import { usePipeline } from "./pipeline";

function App() {
  const { onApply, ...pipeline } = usePipeline();
  return (
    <div className="container">
      <ReactFlow {...pipeline} >
        <MiniMap />
        <Controls />
        <Background />
      </ReactFlow>
      <Button onClick={onApply} primary>Apply</Button>
    </div>
  );
}

export default App;
